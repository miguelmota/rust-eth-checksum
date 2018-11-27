extern crate regex;
extern crate crypto;

use std::env;
use std::process;
use std::i64;
use regex::Regex;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("argument is required");
        process::exit(1);
    }

    let input = &args[1];

    let re = Regex::new(r"^0x").unwrap();
    let address = input.to_lowercase();
    let address = re.replace_all(&address, "").to_string();

    let mut checksum_address = "0x".to_string();
    let mut hasher = Sha3::keccak256();
    hasher.input_str(&address);
    let address_hash = hasher.result_str();

    for i in 0..address.len() {
        let n = i64::from_str_radix(&address_hash.chars().nth(i).unwrap().to_string(), 16).unwrap();
        let ch = address.chars().nth(i).unwrap();
        // make char uppercase if ith character is 9..f
        if n > 7 {
          checksum_address = format!("{}{}", checksum_address, ch.to_uppercase().to_string());
        } else {
          checksum_address = format!("{}{}", checksum_address, ch.to_string());
        }
    }

    println!("{}", checksum_address);
}
