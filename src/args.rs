mod command_line_args;
mod prefix_set;

use std::path::{Path, PathBuf};
use command_line_args::CommandLineArgs;
pub use prefix_set::PrefixSet;

use clap::Parser;

pub struct Args {
    command_line_args: CommandLineArgs,
    prefix_set: PrefixSet,
    folder_absolute: PathBuf,
}

impl Args {
    pub fn new() -> Args {
        let command_line_args = CommandLineArgs::parse();
        let prefix_set: PrefixSet = if command_line_args.ascii {
            PrefixSet {
                parent_prefix: r"|   ".to_string(),
                no_parent_prefix: r"    ".to_string(),
                entry_prefix: r"+-- ".to_string(),
                last_entry_prefix: r"\-- ".to_string(),
            }
         } else {
            PrefixSet {
                parent_prefix: r"│   ".to_string(),
                no_parent_prefix: r"    ".to_string(),
                entry_prefix: r"├── ".to_string(),
                last_entry_prefix: r"└── ".to_string(),
            }
        };
        let folder_path = Path::new(command_line_args.folder.as_str()).to_path_buf();
        let folder_absolute = if folder_path.is_absolute() {
            folder_path
        } else {
            match Args::absolute_path(&folder_path) {
                Ok(f) => f,
                _  => folder_path, // We'll validate the path later...
            }
        };
        Args {
            command_line_args,
            prefix_set,
            folder_absolute,
        }
    }

    pub fn include_hidden(&self) -> bool { self.command_line_args.include_hidden }
    pub fn dirs_only(&self) -> bool { self.command_line_args.dirs_only }
    pub fn level(&self) -> u32 { self.command_line_args.level }
    pub fn path_absolute(&self) -> &PathBuf { &self.folder_absolute }
    pub fn prefix_set(&self) -> &PrefixSet { &self.prefix_set }

    fn absolute_path(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
        let path = path.as_ref();

        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
        match absolute_path.canonicalize() {
            Ok(path) => Ok(path),
            Err(_) => Ok(absolute_path)
        }
    }
}