fn main() {
    let addr = "0xe0fc04fa2d34a66b779fd5cee748268032a146c0";
    let checksummed = eth_checksum::checksum(&addr);

    println!("{}", checksummed);
    assert_eq!("0xe0FC04FA2d34a66B779fd5CEe748268032a146c0", &checksummed);
}
