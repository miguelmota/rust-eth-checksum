#[cfg(test)]
mod tests {
    #[test]
    fn checksum() {
        let addr = "0xe0fc04fa2d34a66b779fd5cee748268032a146c0";
        let checksummed = eth_checksum::checksum(&addr);
        assert_eq!(
            checksummed.to_string(),
            "0xe0FC04FA2d34a66B779fd5CEe748268032a146c0"
        );

        let addr = "0xE0FC04FA2D34A66B779FD5CEE748268032A146C0";
        let checksummed = eth_checksum::checksum(&addr);
        assert_eq!(
            checksummed.to_string(),
            "0xe0FC04FA2d34a66B779fd5CEe748268032a146c0"
        );
    }
}
