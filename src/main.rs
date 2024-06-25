use std::env;
use std::fs::File;
use std::io::Read;
use streebog::{Digest, Streebog256};
use hex::encode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file_path = &args[1];
    let mut file = File::open(file_path).expect("Failed to open file");

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let mut hasher = Streebog256::new();
    hasher.update(&buffer);
    // hasher.update("The quick brown fox jumps over the lazy dog");
    let hash = hasher.finalize();
    let hash256 = encode(&hash);
    println!("Streebog-256 hash: {:?}", hash256);
}
