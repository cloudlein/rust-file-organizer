use std::env;
use crate::scanner::scan;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "file-organizer")]
#[command(about = "Organize files by extension")]
pub struct Cli {

    #[arg(short, long, default_value = "./test_files")]
    pub path: String,

    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,
}


pub fn parse_args() -> Cli{
//    println!("CLI module loaded (placeholder).");

    Cli::parse()
    // let args: Vec<String> = env::args().collect();
    // let path = args.get(1).map(|s| s.as_str()).unwrap_or("./test_files");
    //
    // scan(path);

}
