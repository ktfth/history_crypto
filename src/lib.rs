use hex::ToHex;

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
        let current = d_char_code ^ k_char_code;

        encrypted_message.push(std::char::from_digit(current.into(), 16).unwrap());

        key_charger += 1;

        if key.len() == key_charger {
            key_charger = 0;
        }
    }
    let result = encrypted_message.iter().cloned().collect::<String>();
    result.as_str().encode_hex::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ciphering_xor() {
        assert_eq!(97, char_code("a"));
        assert_eq!("6539646239".to_string(), cipher_xor("hello", "flag"));
    }
}