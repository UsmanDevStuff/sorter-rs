use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    // Prompt the user for the folder path
    println!("Directory Sorter");
    print!("Enter folder path: ");
    io::stdout().flush().unwrap();

    let mut folder_path = String::new();
    io::stdin().read_line(&mut folder_path).unwrap();
    let folder_path = folder_path.trim();

    // Sort the folder contents into individual folders based on file extensions
    let folder = Path::new(folder_path);
    let files = fs::read_dir(&folder).unwrap();

    let mut sorted_folders: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for file in files {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_file() {
                let file_path = file.path();
                if let Some(extension) = file_path.extension() {
                    let extension = extension.to_string_lossy().to_string();
                    sorted_folders
                        .entry(extension)
                        .or_insert_with(Vec::new)
                        .push(file_path);
                } else {
                    sorted_folders
                        .entry("No Extension".to_string())
                        .or_insert_with(Vec::new)
                        .push(file_path);
                }
            }
        }
    }

    // Log all the information
    for (extension, files) in &sorted_folders {
        let folder_name = format!("{} - Files", extension);
        let folder_path = folder.join(&folder_name);

        fs::create_dir_all(&folder_path).unwrap();

        for file in files {
            let new_file_path = folder_path.join(file.file_name().unwrap());
            fs::rename(&file, &new_file_path).unwrap();
        }

        println!("{}: {:?}", folder_name, files);
    }
}
