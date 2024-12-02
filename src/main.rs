use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{mpsc::channel, Arc, Mutex};
use std::time::Duration;

fn organize_file(file_path: &Path) {
    let extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    // Determine the target folder based on file type
    let target_folder_name = match extension {
        "jpg" | "png" | "jpeg" => "Images",
        "pdf" | "docx" | "txt" => "Documents",
        "zip" | "rar" | "7z" => "Archives",
        _ => "Misc",
    };

    let downloads_dir = file_path.parent().unwrap();
    let target_folder = downloads_dir.join(target_folder_name);

    std::fs::create_dir_all(&target_folder).expect("Failed to create folder");
    let target_path = target_folder.join(file_path.file_name().unwrap());

    std::fs::rename(file_path, &target_path).expect("Failed to move file");
    println!("Moved file {:?} to {:?}", file_path, target_folder);
}

fn main() -> notify::Result<()> {
    // Create a channel to receive file system events
    let (tx, rx) = channel();

    // Use a HashSet to track processed files
    let processed_files: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));

    // Create a RecommendedWatcher and send events to the channel
    let processed_files_clone = Arc::clone(&processed_files);
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            tx.send(res).expect("Failed to send file system event");
        },
        Config::default().with_poll_interval(Duration::from_secs(2)), // Optional: Poll interval
    )?;

    // Specify the folder to watch
    let folder_to_watch = "C:\\Users\\panue\\Downloads";
    watcher.watch(Path::new(folder_to_watch), RecursiveMode::NonRecursive)?;

    println!("Watching folder: {}", folder_to_watch);

    for res in rx {
        match res {
            Ok(event) => {
                if let Some(path) = event.paths.get(0) {
                    let mut processed = processed_files_clone.lock().unwrap();

                    // Skip processing if the file was already handled
                    if processed.contains(path) {
                        println!("Skipping already processed file: {:?}", path);
                        continue;
                    }

                    println!("File system event: {:?}", event);

                    if let EventKind::Create(_) = event.kind {
                        println!("New file detected: {:?}", path);
                        organize_file(path);

                        // Mark the file as processed
                        processed.insert(path.clone());
                    }
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
