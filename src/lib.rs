#![allow(unused)]

use std::error::Error;
use std::fs;

pub mod sha256;
pub mod md5;

#[derive(Debug)]
pub enum HashType {
    MD5,
    SHA256
}

pub struct Config {
    hash_type: HashType,
    file_paths: Vec<String>
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("No files specified");
        }

        let mut hash_type = HashType::MD5;
        let mut file_paths = Vec::new();

        for (index, arg) in args.iter().skip(1).enumerate() {
            if (args[index] == "--hash") || (args[index] == "-h") {
                continue;
            }

            if (arg == "--hash") || (args[index] == "-h") {
                hash_type = match args[index + 2].to_lowercase().as_str() {
                    "md5" => HashType::MD5,
                    "sha256" => HashType::SHA256,
                    _ => return Err("Invalid hash type specified"),
                }
            } else {
                file_paths.push(arg.clone());
            }
        }

        if file_paths.len() == 0 { return Err("No files specified") }

        Ok(Config { hash_type, file_paths })
    }
}

pub enum HashResult {
    MD5([u8; 16]),
    SHA256([u8; 32])
}

pub fn hash_to_hex_string(h: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in h {
        hex_string.push_str(&format!("{:02x}", byte));
    }
    hex_string
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_count = config.file_paths.len();
    eprintln!("Computing {:?} checksum for {} file{}...", config.hash_type, file_count, {
        match file_count {
            1 => "",
            _ => "s"
        }
    });

    let mut hashes: Vec<(String, HashResult)> = Vec::new();

    for file_path in config.file_paths {
        let bytes = fs::read(&file_path)?;
        let hash = match config.hash_type {
            HashType::MD5 => md5::md5(&bytes),
            HashType::SHA256 => sha256::sha256(&bytes), 
        };

        hashes.push((file_path, hash));
    }

    //output results
    for (file_path, hash) in hashes {
        match hash {
            HashResult::SHA256(h) => {
                println!("\"{}\": {}", file_path, hash_to_hex_string(&h));
            },
            HashResult::MD5(h) => {
                println!("\"{}\": {}", file_path, hash_to_hex_string(&h));
            }
        }
    }
    

    Ok(())
}