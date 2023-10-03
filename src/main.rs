use clap::{arg, command, Parser, Subcommand};
use std::{env, fs, fs::File, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Memorize {
    #[arg(short, long)]
    alias: String,

    #[command(subcommand)]
    command: MemorizeSubcommands,
}

#[derive(Subcommand, Debug)]
enum MemorizeSubcommands {
    /// Adding and memorize command
    Add { cmd: String },
    /// Delete the specific memorized command
    Del { cmd: String },
    /// Update the specific memorized command
    Set { cmd: String },
    /// Execute the target memorized command by its alias
    Use { alias: String },
    /// Show a list of memorized commands and its alias
    List { list: usize },
}

const HOME: &str = "HOME";
const DEFAULT_PATH: &str = ".local/share/mem";

fn main() {
    let args = Memorize::parse();
}

pub fn write_into(file_name: &str) -> Result<(), std::io::Error> {
    let home_dir = env::var(HOME).expect("should been read `HOME` value");
    let path = format!("{home_dir}/{DEFAULT_PATH}");
    let file_path = format!("{home_dir}/{DEFAULT_PATH}/{file_name}");
    let is_dir = Path::new(&path).is_dir();
    let is_file = Path::new(&file_path).is_file();
    if !is_dir {
        fs::create_dir(&path)?;
    }
    if !is_file {
        File::create(path)?;
    }
    Ok(())
}
