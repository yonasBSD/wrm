use crate::{Error::NotFound, FilesInTrash, Result};
use colored::Colorize;
use filey::FileTypes;
use std::fmt::Display;

pub fn list(_files_in_trash: &FilesInTrash) -> Result<()> {
    let trash_items = trash::os_limited::list().unwrap();

    if trash_items.is_empty() {
        eprintln!("There are no files or directories in trash");

        return Ok(());
    }

    for file in trash_items {
        println!("{}", file.original_path().display());
        //let colorized_path = colorize(file.name);
        //let formatted = format(colorized_path, file.original_path().display());
        //println!("{}", formatted);
    }

    Ok(())
}

fn format<D: Display, E: Display>(path: D, from: E) -> String {
    format!("{} (from {})", path, from)
}

fn colorize<D: Display>(path: D) -> Result<String> {
    let not_found = NotFound {
        path: path.to_string(),
    };

    match FileTypes::which(path.to_string()).ok_or(not_found)? {
        FileTypes::File => Ok(path.to_string()),
        FileTypes::Directory => Ok(path.to_string().blue().to_string()),
        FileTypes::Symlink => Ok(path.to_string().cyan().to_string()),
    }
}
