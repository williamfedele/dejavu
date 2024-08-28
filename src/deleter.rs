use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn delete(
    duplicates: HashMap<String, Vec<PathBuf>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for (_, paths) in duplicates {
        let options: Vec<String> = paths
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();

        let selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select files to delete (space). Enter to confirm.")
            .items(&options)
            .interact()?;

        if !selection.is_empty() {
            for &index in &selection {
                let path = &paths[index];
                match std::fs::remove_file(path) {
                    Ok(_) => println!("Deleted {}", path.display()),
                    Err(e) => eprintln!("Failed to delete {}: {}", path.display(), e),
                }
            }
        } else {
            println!("No files selected to delete.")
        }

        println!()
    }

    Ok(())
}
