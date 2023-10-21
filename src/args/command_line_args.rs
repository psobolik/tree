use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct CommandLineArgs {
    #[arg(short('a'), long("all"), help = "List all files and/or directories, including those that start with '.'")]
    pub include_hidden: bool,
    #[arg(short, long, help = "List directories only")]
    pub dirs_only: bool,
    #[arg(long, help = "Use ASCII instead of extended characters")]
    pub ascii: bool,
    #[arg(short('L'), default_value_t = 0, help = "List maximum levels deep (0 for no limit)")]
    pub level: u32,

    #[arg(default_value = ".")]
    pub folder: String,
}


