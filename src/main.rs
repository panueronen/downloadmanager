mod config;
use config::{default_config, load_config, save_config, FileRule};
use notify::{
    Config as NotifyConfig, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    sync::mpsc::channel,
    time::Duration,
};

const TEMPORARY_EXTENSIONS: &[&str] = &["tmp", "crdownload", "part", "swp"];
const BUFFER_SIZE: usize = 1024;

fn calculate_file_hash(file_path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; BUFFER_SIZE];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn ensure_folder_exists(folder: &Path) {
    fs::create_dir_all(folder).unwrap_or_else(|_| panic!("Failed to create folder: {:?}", folder));
}

fn is_duplicate(file_path: &Path, target_folder: &Path) -> bool {
    if let Ok(file_hash) = calculate_file_hash(file_path) {
        if let Ok(entries) = fs::read_dir(target_folder) {
            for existing_file in entries.flatten() {
                if let Ok(existing_hash) = calculate_file_hash(&existing_file.path()) {
                    if file_hash == existing_hash {
                        log::info!(
                            "Duplicate detected: {:?} (original: {:?})",
                            file_path,
                            existing_file.path()
                        );
                        return true;
                    }
                }
            }
        } else {
            log::warn!("Failed to read target folder: {:?}", target_folder);
        }
    }
    false
}

fn handle_duplicate(file_path: &Path, downloads_dir: &Path) {
    let duplicates_folder = downloads_dir.join("Duplicates");
    ensure_folder_exists(&duplicates_folder);
    let target_path = duplicates_folder.join(file_path.file_name().unwrap());
    fs::rename(file_path, &target_path).expect("Failed to move duplicate file");
    log::info!("Moved duplicate file to: {:?}", target_path);
}

fn organize_file(file_path: &Path, file_rules: &[FileRule]) {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    if TEMPORARY_EXTENSIONS.contains(&extension) {
        log::info!("Skipping temporary file: {:?}", file_path);
        return;
    }

    if let Some(rule) = file_rules.iter().find(|rule| rule.extension == extension) {
        let downloads_dir = file_path.parent().unwrap();
        let target_folder = downloads_dir.join(&rule.folder);

        ensure_folder_exists(&target_folder);

        if is_duplicate(file_path, &target_folder) {
            handle_duplicate(file_path, downloads_dir);
        } else {
            let target_path = target_folder.join(file_path.file_name().unwrap());
            fs::rename(file_path, &target_path).expect("Failed to move file");
            log::info!("Moved file to: {:?}", target_path);
        }
    } else {
        log::warn!(
            "No rule found for file extension: {}. Skipping file: {:?}",
            extension,
            file_path
        );
    }
}

fn main() -> notify::Result<()> {
    env_logger::init();

    let config_path = Path::new("config.toml");

    let config = if config_path.exists() {
        load_config(config_path)
    } else {
        let default = default_config();
        save_config(config_path, &default);
        log::info!("Default configuration created at {:?}", config_path);
        default
    };

    log::info!("Loaded configuration: {:?}", config);

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            tx.send(res).expect("Failed to send file system event");
        },
        NotifyConfig::default().with_poll_interval(Duration::from_secs(2)),
    )?;

    let root_folder = Path::new(&config.folder_to_watch);
    watcher.watch(root_folder, RecursiveMode::NonRecursive)?;

    log::info!("Watching folder: {}", config.folder_to_watch);

    for res in rx {
        match res {
            Ok(event) => {
                if let Some(path) = event.paths.first() {
                    if path.parent() != Some(root_folder) {
                        log::info!("Ignoring event from subfolder: {:?}", path);
                        continue;
                    }

                    log::info!("File system event: {:?}", event);

                    if let EventKind::Create(_) = event.kind {
                        log::info!("New file detected: {:?}", path);
                        organize_file(path, &config.file_rules);
                    }
                }
            }
            Err(e) => log::error!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
