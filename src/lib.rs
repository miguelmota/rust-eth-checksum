use crypto::{digest::Digest, sha3::Sha3};

pub fn checksum(address: &str) -> String {
    let address = address.trim_start_matches("0x").to_lowercase();

    let mut hasher = Sha3::keccak256();
    hasher.input_str(&address);
    let address_hash = hasher.result_str();

    address.chars().zip(address_hash.chars()).fold(
        String::from("0x"),
        |mut acc, (address_char, hash_char)| {
            let n = u16::from_str_radix(&hash_char.to_string(), 16).unwrap();

            if n > 7 {
                // make char uppercase if ith character is 9..f
                acc.push_str(&address_char.to_uppercase().to_string())
            } else {
                // already lowercased
                acc.push(address_char)
            }

            acc
        },
    )
}
