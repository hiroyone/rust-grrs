use std::{time::Duration, thread};

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
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        thread::sleep(Duration::from_secs(2));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}



