use num_format::{Locale, ToFormattedString};

pub struct Counts {
    directories: u32,
    files: u32,
}

impl Counts {
    pub fn increment_directories(&mut self) {
        self.directories += 1;
    }

    pub fn increment_files(&mut self) {
        self.files += 1;
    }

    fn format_directories(&self) -> String {
        format!(
            "{} {}",
            self.directories.to_formatted_string(&Locale::en),
            Counts::select_plural(self.directories, "directory", "directories")
        )
    }

    fn format_files(&self) -> String {
        format!(
            "{} {}",
            self.files.to_formatted_string(&Locale::en),
            Counts::select_plural(self.files, "file", "files")
        )
    }
}
impl Counts {
    pub fn new() -> Counts {
        Counts {
            directories: 0,
            files: 0,
        }
    }

    fn select_plural<'a>(value: u32, singular: &'a str, plural: &'a str) -> &'a str {
        if value == 1 {
            singular
        } else {
            plural
        }
    }
}

impl std::fmt::Display for Counts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.format_directories(), self.format_files(),)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_has_zero_directories_and_zero_files() {
        let counts = Counts::new();
        assert!(counts.directories == 0 && counts.files == 0)
    }

    #[test]
    fn increment_directories_icrements_directories() {
        let mut counts = Counts::new();
        counts.increment_directories();
        assert!(counts.directories == 1 && counts.files == 0)
    }

    #[test]
    fn increment_files_increments_files() {
        let mut counts = Counts::new();
        counts.increment_files();
        assert!(counts.directories == 0 && counts.files == 1)
    }

    #[test]
    fn multiple_increments_increments_multiple_times() {
        let mut counts = Counts::new();
        counts.increment_files();
        counts.increment_files();
        counts.increment_files();
        counts.increment_directories();
        counts.increment_directories();
        counts.increment_directories();
        counts.increment_directories();
        counts.increment_directories();
        assert!(counts.directories == 5 && counts.files == 3)
    }

    #[test]
    fn thousands_are_formatted_with_separators() {
        let counts = Counts {
            directories: 1234,
            files: 9876,
        };
        assert_eq!(counts.to_string(), "1,234 directories, 9,876 files")
    }

    #[test]
    fn display_handles_singular_values_correctly() {
        let counts = Counts {
            directories: 1,
            files: 1,
        };
        assert_eq!(counts.to_string(), "1 directory, 1 file")
    }

    #[test]
    fn display_handles_plural_values_correctly() {
        let counts = Counts {
            directories: 0,
            files: 10,
        };
        assert_eq!(counts.to_string(), "0 directories, 10 files")
    }
}
