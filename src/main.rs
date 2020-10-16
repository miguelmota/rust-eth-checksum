use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.get(1) {
        Some(input) => {
            let checksummed_address = eth_checksum::checksum(input);

            println!("{}", checksummed_address);
        }
        None => {
            println!("argument is required");
            process::exit(1);
        }
    }
}
