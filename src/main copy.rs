use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Get the current working directory.
    let current_dir = env::current_dir()?;
    println!("Operating in directory: {:?}", current_dir);

    // Iterate over each entry in the current directory.
    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a directory.
        if path.is_dir() {
            // Attempt to get the folder name as a string.
            if let Some(folder_name) = path.file_name().and_then(|name| name.to_str()) {
                // If the folder name already starts with "DISABLED_", skip renaming.
                if folder_name.starts_with("DISABLED_") {
                    println!("Skipping folder '{}' because it is already disabled.", folder_name);
                    continue;
                }

                // Create the new folder name by adding the prefix.
                let new_name = format!("DISABLED_{}", folder_name);
                let new_path = path.with_file_name(new_name);

                // Rename the folder.
                println!("Renaming folder '{}' to '{:?}'", folder_name, new_path.file_name().unwrap());
                fs::rename(&path, &new_path)?;
            }
        } else {
            // For non-directory entries, simply print a message and skip.
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                println!("Skipping non-directory entry: {}", name);
            }
        }
    }

    println!("Renaming complete.");
    Ok(())
}
