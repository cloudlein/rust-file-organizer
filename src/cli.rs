use std::env;
use crate::scanner::scan;
use clap::{Command, Parser, Subcommand};
use crate::cli::Commands::Scan;

#[derive(Parser, Debug)]
#[command(name = "file-organizer")]
#[command(about = "Organize files by extension")]
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands,

}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Scan {
        #[arg(short, long)]
        path: String,
    },

    Organize{
        #[arg(short, long)]
        path: String,

        #[arg(short, long, default_value_t = false)]
        dry_run: bool,
    }
}

pub fn parse_args() -> Cli{
//    println!("CLI module loaded (placeholder).");

    Cli::parse()
    // let args: Vec<String> = env::args().collect();
    // let path = args.get(1).map(|s| s.as_str()).unwrap_or("./test_files");
    //
    // scan(path);

}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parse_args() {
        let args = Cli::try_parse_from([
            "rust-file-organizer",
            "scan",
            "--path",
            "./test_files"
        ]).unwrap();


        match args.command {
            Commands::Scan { path } => {
                assert_eq!(path, "./test_files");
            }
            _ => panic!("Expected scan command!")
        }
    }

    #[test]
    fn test_organize_command() {
        let args = Cli::try_parse_from([
            "rust-file-organizer",
            "organize",
            "--path",
            "./test_files",
            "--dry-run"
        ]).unwrap();


        match args.command {
            Commands::Organize { path , dry_run } => {
                assert_eq!(path, "./test_files");
                assert_eq!(dry_run, true);
            }
            _ => panic!("Expected scan command!")
        }
    }

    #[test]
    fn test_invalid_args() {
        let args = Cli::try_parse_from([
            "rust-file-organizer",
            "scan",
            "-path",
            "./test_files",
        ]);

        assert!(args.is_err());
    }

    #[test]
    fn test_missing_path() {
        let args = Cli::try_parse_from([
            "rust-file-organizer",
            "scan",
        ]);

        assert!(args.is_err());
    }

    #[test]
    fn test_organize_without_dry_run() {
        let args = Cli::try_parse_from([
            "rust-file-organizer",
            "organize",
            "--path",
            "./test_files",
        ]).unwrap();

        match args.command {
            Commands::Organize { path, dry_run } => {
                assert_eq!(path, "./test_files");
                assert!(!dry_run);
            }
            _ => panic!("Expected organize command!")
        }
    }

}