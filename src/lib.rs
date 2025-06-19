pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let version_bytes = match hex::decode(&raw_tx_hex[..8]) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Hex decode error".to_string()),
    };

    let version = u32::from_le_bytes(version_bytes.try_into().unwrap());
    Ok(version)
}
