use crate::{
    exists_or_else, utils::*, Error::NotFound, FilesInTrash, Options, Result, FILES_IN_TRASH,
};
use colored::Colorize;
use filey::Filey;
use std::fmt::Display;
use trash;

pub fn restore(
    paths: &Vec<String>,
    files_in_trash: &mut FilesInTrash,
    options: &Options,
) -> Result<()> {
    for path in paths {
        let path = &absolutize(path)?;

        //exists_or_else!(path);

        restore_inner(path, files_in_trash, options)?;
    }

    Ok(())
}

fn restore_inner(
    path: &String,
    files_in_trash: &mut FilesInTrash,
    options: &Options,
) -> Result<()> {
    for file in trash::os_limited::list().unwrap() {
        if asref_path_to_string(path) == asref_path_to_string(file.original_path()) {
            let f1 = file.name.clone();
            let f2 = file.name.clone();
            let f3 = file.name.clone();

            if !options.noninteractive() {
                let message = format!(
                    "{} '{}' to '{}'?",
                    "Restore".red().bold(),
                    asref_path_to_string(file.original_path()),
                    f1
                );

                if !confirm(message)? {
                    eprintln!("Canceled");

                    break;
                }
            }

            if let Err(e) = rename(file.original_path(), f2) {
                eprintln(e);

                break;
            }

            trash::os_limited::restore_all(trash::os_limited::list().unwrap().into_iter().filter(|x| x.name == f3)).unwrap();

            print_log_message(asref_path_to_string(file.original_path()), file.name, options.quiet());

            break;
        }
    }

    Ok(())
}

fn print_log_message<D: Display, E: Display>(path: D, from: E, quiet: bool) {
    if !quiet {
        eprintln!("{} '{}' to '{}'", "Restored".green().bold(), path, from);
    }
}
