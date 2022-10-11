use std::{io::{BufReader, BufRead}, fs::File};
use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() {

    let args = Cli::parse();
    let f = File::open(&args.path).expect("could not read file");
    let content = BufReader::new(f);

    for line in content.lines() {  
        let unwrapped_line = line.unwrap();
        if unwrapped_line.contains(&args.pattern) {
            println!("{}", unwrapped_line);
        }
    }

}
