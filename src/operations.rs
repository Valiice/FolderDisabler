use std::fs;
use std::io;
use std::path::Path;

/// Adds the "DISABLED_" prefix to all folders in the given directory that do not already have it.
pub fn add_prefix(current_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Process only directories.
        if path.is_dir() {
            // Attempt to get the folder name as a &str.
            if let Some(folder_name) = path.file_name().and_then(|name| name.to_str()) {
                // Skip if the folder name already starts with "DISABLED_".
                if folder_name.starts_with("DISABLED_") {
                    println!("Skipping folder '{}' because it is already disabled.", folder_name);
                    continue;
                }

                // Create the new folder name by adding the prefix.
                let new_name = format!("DISABLED_{}", folder_name);
                let new_path = path.with_file_name(new_name);
                println!(
                    "Renaming folder '{}' to '{}'",
                    folder_name,
                    new_path.file_name().unwrap().to_str().unwrap()
                );
                fs::rename(&path, &new_path)?;
            }
        } else {
            // For non-directory entries, simply print a message and skip.
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                println!("Skipping non-directory entry: {}", name);
            }
        }
    }
    Ok(())
}

/// Removes the "DISABLED_" prefix from all folders in the given directory that have it.
pub fn remove_prefix(current_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Process only directories.
        if path.is_dir() {
            if let Some(folder_name) = path.file_name().and_then(|name| name.to_str()) {
                // Only remove the prefix if it exists.
                if folder_name.starts_with("DISABLED_") {
                    let new_name = folder_name.trim_start_matches("DISABLED_").to_string();
                    let new_path = path.with_file_name(new_name);
                    println!(
                        "Renaming folder '{}' to '{}'",
                        folder_name,
                        new_path.file_name().unwrap().to_str().unwrap()
                    );
                    fs::rename(&path, &new_path)?;
                } else {
                    println!("Skipping folder '{}' because it doesn't have the prefix.", folder_name);
                }
            }
        } else {
            // For non-directory entries, simply print a message and skip.
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                println!("Skipping non-directory entry: {}", name);
            }
        }
    }
    Ok(())
}
