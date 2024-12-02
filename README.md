# **Rust File Organizer**

Rust File Organizer is a lightweight application that automatically organizes files in a specified folder (e.g., your `Downloads` directory). It categorizes files based on their extensions, handles duplicate files, and runs efficiently in the background.

---

## **Features**

- ✅ Automatically organizes files into predefined categories (e.g., `Documents`, `Images`, `Audio`).
- ✅ Detects and moves duplicate files to a `Duplicates` folder.
- ✅ Ignores temporary or partial files (e.g., `.tmp`, `.crdownload`).
- ✅ Configurable rules via a simple `config.toml` file.
- ✅ Works silently in the background with no console window.

---

## **Getting Started**

### **1. Installation**

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/<your-username>/rust-file-organizer.git
   cd rust-file-organizer
   ```

2. **Build the Application**:
   ```bash
   cargo build --release
   ```

3. **Locate the Executable**:
   - The compiled binary will be located in `target/release`.

---

### **2. Configuration**

Before running the application, configure the organization rules in `config.toml`:

1. **Create/Edit `config.toml`**:
   - The program looks for `config.toml` in the same directory as the executable. Below is an example configuration:

     ```toml
     folder_to_watch = "C:\\Users\\YourUsername\\Downloads"

     [[file_rules]]
     extension = "jpg"
     folder = "Images"

     [[file_rules]]
     extension = "png"
     folder = "Images"

     [[file_rules]]
     extension = "pdf"
     folder = "Documents"

     [[file_rules]]
     extension = "mp3"
     folder = "Audio"

     [[file_rules]]
     extension = "*"
     folder = "Misc"
     ```

   - **Key Fields**:
     - `folder_to_watch`: Path to the folder being monitored.
     - `file_rules`: Define extensions and corresponding target folders.

2. **Default Behavior**:
   - Files without a matching rule are moved to the `Misc` folder.
   - Duplicate files are moved to a `Duplicates` folder.

---

### **3. Running the Application**

- To run the application, simply execute the binary:
  ./target/release/rust-file-organizer

- **Run Without a Console Window (Windows)**:
  - The program suppresses the console window when compiled with the `windows` subsystem. Ensure the build and configuration are correct.
