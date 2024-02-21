use crate::{Error::WrmError, Result};
use colored::Colorize;
use filey::Filey;
use inquire::Confirm;
use std::{error::Error, fmt::Display, path::Path};
use trash;
use home;

pub fn absolutize<P: AsRef<Path>>(path: P) -> Result<String> {
    let absolutized = Filey::new(path)
        .expand_user()
        .map_err(|e| e.into())
        .map_err(WrmError)?
        .absolutize()
        .map_err(|e| e.into())
        .map_err(WrmError)?
        .to_string();

    Ok(absolutized)
}

pub fn file_name<P: AsRef<Path>>(path: P) -> Option<String> {
    let file_name = Filey::new(path).file_name()?;

    Some(file_name)
}

pub fn remove<P: AsRef<Path>>(path: P) -> Result<()> {
    /*
    println!("Items in trash");
    for item in trash::os_limited::list().unwrap() {
        println!("{}", item.original_path().display());
    }
    */

    let p: String = asref_path_to_string(path);
    //println!("Trying to remove {:#?}", p);

    let selected: Vec<_>;
    if p != format!("{}/.config/wrm", home::home_dir().unwrap().display()) {
        selected = trash::os_limited::list().unwrap().into_iter().filter(|x| x.name == p).collect();
    } else {
        selected = trash::os_limited::list().unwrap();
    }

    trash::os_limited::purge_all(selected).unwrap();
    Ok(())
}

pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, _to: Q) -> Result<()> {
    let _ = trash::delete(from);

    Ok(())
}

pub fn confirm<D: Display>(message: D) -> Result<bool> {
    Confirm::new(message.to_string().as_str())
        .with_default(false)
        .prompt()
        .map_err(|e| e.into())
        .map_err(WrmError)
}

pub fn asref_path_to_string<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_string_lossy().to_string()
}

pub fn eprintln<E: Error>(error: E) {
    eprintln!("{}: {}", "error".red().bold(), error);
}

#[macro_export]
macro_rules! exists_or_else {
    ( $path:expr ) => {
        if !Filey::new($path).exists() {
            let not_found = NotFound {
                path: $path.to_string(),
            };

            eprintln(not_found);

            continue;
        }
    };
}
