use std::env;
use crate::scanner::scan;

pub fn parse_args() {
//    println!("CLI module loaded (placeholder).");

    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or("./test_files");

    scan(path);

}
