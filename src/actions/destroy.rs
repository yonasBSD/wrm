use crate::{exists_or_else, utils::*, Error::NotFound, Options, Result};
use colored::Colorize;
use filey::Filey;

pub fn destroy(paths: &Vec<String>, options: &Options) -> Result<()> {
    for path in paths {
        exists_or_else!(path);

        if !options.noninteractive() {
            let message = format!("{} '{}'?", "Destroy".red().bold(), path);

            if !confirm(message)? {
                eprintln!("Canceled");

                continue;
            }
        }

        if let Err(e) = remove(path) {
            eprintln(e);

            continue;
        }

        if !options.quiet() {
            eprintln!("{} '{}'", "Destroyed".green().bold(), path);
        }
    }

    Ok(())
}
