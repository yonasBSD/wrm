use crate::{subcommands::list, utils::*, FilesInTrash, Options, Result, WRM_PATH};
use colored::Colorize;
use trash;

pub fn empty(files_in_trash: &FilesInTrash, options: &Options) -> Result<()> {
    if trash::os_limited::list().unwrap().is_empty() {
        eprintln!("There are no files or directories in trash");

        return Ok(());
    }

    if !options.noninteractive() {
        list(files_in_trash)?;

        let message = format!("{} trash?", "Empty".red().bold());

        if !confirm(message)? {
            eprintln!("Canceled");

            return Ok(());
        }
    }

    remove(absolutize(WRM_PATH)?)?;

    if !options.quiet() {
        eprintln!("{} trash", "Emptied".green().bold());
    }

    Ok(())
}
