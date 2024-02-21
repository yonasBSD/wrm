use crate::{exists_or_else, utils::*, Error::NotFound, Options, Result};
use colored::Colorize;
use filey::Filey;

pub fn delete(paths: &Vec<String>, options: &Options) -> Result<()> {
    for path in paths {
        let opath = path;
        let path = absolutize(path)?;
        //exists_or_else!(path.clone());

        if !options.noninteractive() {
            let message = format!("{} '{}'?", "Delete".red().bold(), path.clone());

            if !confirm(message)? {
                eprintln!("Canceled");
                continue;
            }
        }

        if let Err(e) = remove(opath.clone()) {
            eprintln(e);

            continue;
        }

        if !options.quiet() {
            eprintln!("{} '{}'", "Deleted".green().bold(), path.clone());
        }
    }

    Ok(())
}
