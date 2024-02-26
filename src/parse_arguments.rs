use crate::{actions, utils::*, FilesInTrash, Result, FILES_IN_TRASH};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
    verbatim_doc_comment
)]
struct Arguments {
    paths: Option<Vec<String>>,
    /// Destroy files.
    #[clap(short, long)]
    destroy: bool,
    /// Empty trash.
    #[clap(short, long)]
    empty: bool,
    /// List files in trash.
    #[clap(short, long)]
    list: bool,
    /// Do not prompt whether to change destinations.
    #[clap(short, long)]
    noninteractive: bool,
    /// Do not print log messages.
    #[clap(short, long)]
    quiet: bool,
    /// Restore files to where they came from.
    #[clap(short, long)]
    restore: bool,
}

#[derive(Debug)]
pub struct Options {
    noninteractive: bool,
    quiet: bool,
}

impl Options {
    pub const fn new(noninteractive: bool, quiet: bool) -> Self {
        Self {
            noninteractive,
            quiet,
        }
    }

    pub const fn noninteractive(&self) -> bool {
        self.noninteractive
    }

    pub const fn quiet(&self) -> bool {
        self.quiet
    }
}

pub fn parse_arguments() -> Result<()> {
    let arguments = Arguments::parse();

    let options = Options::new(arguments.noninteractive, arguments.quiet);

    let mut files_in_trash = FilesInTrash::read(absolutize(FILES_IN_TRASH)?)?;

    if arguments.destroy {
        if let Some(paths) = arguments.paths {
            return actions::destroy(&paths, &options);
        }
    }

    if arguments.empty {
        return actions::empty(&files_in_trash, &options);
    }

    if arguments.list {
        return actions::list(&files_in_trash);
    }

    if arguments.restore {
        if let Some(paths) = arguments.paths {
            return actions::restore(&paths, &mut files_in_trash, &options);
        }
    }

    if let Some(paths) = arguments.paths {
        return actions::remove(&paths, &mut files_in_trash, &options);
    }

    Ok(())
}
