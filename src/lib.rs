use hex;

pub fn char_code(value: &str) -> u16 {
    let v: Vec<u16> = value.encode_utf16().collect();
    v[0]
}

pub fn cipher_xor(message: &str, key: &str) -> String {
    let mut key_charger = 0;
    let mut encrypted_message: Vec<char> = Vec::new();

    for chr in message.chars() {
        let d = chr.to_string();
        let key_sequence: Vec<char> = key.to_string().chars().collect();
        let k = key_sequence[key_charger].to_string();
        let d_char_code = char_code(&d);
        let k_char_code = char_code(&k);
        let current = d_char_code as u8 ^ k_char_code as u8;

        encrypted_message.push(current as char);

        key_charger += 1;

        if key.len() == key_charger {
            key_charger = 0;
        }
    }
    let result = encrypted_message.iter().cloned().collect::<String>();
    hex::encode(result)
}

pub fn decipher_xor(hash: &str, key: &str) -> String {
    let mut key_charger = 0;
    let mut decrypted_message: Vec<_> = Vec::new();
    let hash_bytes = hex::decode(hash).unwrap();

    for d in hash_bytes {
        let k_data = [key.as_bytes()[key_charger]];
        let k = std::str::from_utf8(&k_data).unwrap();
        let k_char_code = char_code(&k);
        let current = d ^ k_char_code as u8;

        decrypted_message.push(current as char);

        key_charger += 1;

        if key.len() == key_charger {
            key_charger = 0;
        }
    }
    let result = decrypted_message.iter().cloned().collect::<String>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ciphering_xor() {
        assert_eq!(97, char_code("a"));
        assert_eq!("0e090d0b09".to_string(), cipher_xor("hello", "flag"));
    }

    #[test]
    fn deciphering_xor() {
        assert_eq!("hello", decipher_xor("0e090d0b09", "flag"));
    }
}