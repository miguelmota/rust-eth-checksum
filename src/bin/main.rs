use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("argument is required");
        process::exit(1);
    }

    let input = &args[1];
    let checksummed_address = eth_checksum::checksum(input);

    println!("{}", checksummed_address);
}
