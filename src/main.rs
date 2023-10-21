mod args;
mod counts;

use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::args::{Args, PrefixSet};
use counts::Counts;

fn main() {
    let args = Args::new();
    let mut level_flags: Vec<bool> = vec![];
    let mut counts = Counts::new();

    println!("{}", args.path_absolute().display());
    print_tree_level(
        args.path_absolute(),
        &args,
        0,
        &mut counts,
        &mut level_flags,
    );
    println!("\n{counts}");
}

fn print_tree_level(
    path: &PathBuf,
    args: &Args,
    level: u32,
    counts: &mut Counts,
    level_flags: &mut Vec<bool>,
) {
    fn filter_dir_entries(
        dir_entries: fs::ReadDir,
        all: bool,
        dirs_only: bool,
    ) -> Vec<Result<DirEntry, std::io::Error>> {
        dir_entries
            .filter(|dir_entry| match dir_entry {
                Ok(dir_entry) => {
                    (all || !dir_entry.file_name().to_string_lossy().starts_with('.'))
                        && (!dirs_only || dir_entry.path().is_dir())
                }
                Err(_why) => false,
            })
            .collect()
    }
    fn get_prefix(level_flags: &Vec<bool>, is_last_entry: bool, prefix_set: &PrefixSet) -> String {
        let mut result = String::new();
        for level_flag in level_flags {
            result += if *level_flag {
                prefix_set.parent_prefix.as_str()
            } else {
                prefix_set.no_parent_prefix.as_str()
            }
        }
        result += if is_last_entry {
            prefix_set.last_entry_prefix.as_str()
        } else {
            prefix_set.entry_prefix.as_str()
        };
        result
    }
    match fs::read_dir(path) {
        Ok(dir_entries) => {
            let dir_entries =
                filter_dir_entries(dir_entries, args.include_hidden(), args.dirs_only());
            let last_entry_index = if dir_entries.is_empty() {
                0
            } else {
                dir_entries.len() - 1
            };
            (0..dir_entries.len()).for_each(|index| match &dir_entries[index] {
                Ok(dir_entry) => {
                    let dir_entry_path = dir_entry.path();
                    println!(
                        "{}{}",
                        get_prefix(level_flags, index == last_entry_index, args.prefix_set()),
                        dir_entry.file_name().to_string_lossy(),
                    );

                    if dir_entry_path.is_dir() {
                        counts.increment_directories();
                        level_flags.push(true);
                        if index == last_entry_index {
                            if let Some(flag) = level_flags.last_mut() {
                                *flag = false
                            };
                        }
                        let next_level = level + 1;
                        if args.level() == 0 || next_level < args.level() {
                            print_tree_level(
                                &dir_entry_path,
                                args,
                                next_level,
                                counts,
                                level_flags,
                            );
                        }
                        level_flags.pop();
                    } else {
                        counts.increment_files();
                    }
                }
                Err(why) => eprintln!("Error: Can't access '{:?}': {why:?}", dir_entries[index]),
            });
        }
        Err(why) => {
            eprintln!("Error: Can't read directory '{}': {why:?}", path.display());
        }
    }
}
