use clap::Parser;
use anyhow::{Context, Result};
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    use std::io::{self, Write};

    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
    
}



