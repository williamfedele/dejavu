use std::{error::Error, fs};
use std::path::Path;
use std::collections::HashMap;
use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Select};

#[cfg(windows)]
use std::os::windows::fs::symlink_file;

#[cfg(unix)]
use std::os::unix::fs::symlink;

pub fn symlink_files(duplicates: &HashMap<String, Vec<PathBuf>>) -> Result<(), Box<dyn Error>> {
    for (_, paths) in duplicates {
        let options: Vec<String> = paths
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select file to keep as the original (enter to confirm)")
            .items(&options)
            .interact()?;
        
        let original = &paths[selection];
        for (i, path) in paths.iter().enumerate() {
            if i == selection {
                continue;
            }
            create_symlink(original, path)?;
        }
    }
    Ok(())
}

fn create_symlink<P: AsRef<Path>>(original: P, link: P) -> Result<(), Box<dyn Error>> {

    let link = link.as_ref(); 
    if link.exists() {
        fs::remove_file(link)?;
    }

    #[cfg(unix)]
    {
        symlink(original, link)?;
    }

    #[cfg(windows)]
    {
        symlink_file(original, link)
    }
    Ok(())
}
