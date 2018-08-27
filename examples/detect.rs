extern crate chardet;

use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: detect FILE");
        std::process::exit(1);
    }

    // Open text file
    let mut file = OpenOptions::new().read(true).open(&args[1])
        .expect("Could not open file");

    let mut reader: Vec<u8> = Vec::new();

    file.read_to_end(&mut reader)
        .expect("Could not read file");

    // Detect charset of file
    let result = chardet::detect(&reader);

    println!("Detected {:?}", result);
}
