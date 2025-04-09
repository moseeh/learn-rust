#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_lowercase() {
                    (b'a' + b'z' - c as u8) as char
                } else {
                    (b'A' + b'Z' - c as u8) as char
                }
            } else {
                c
            }
        })
        .collect::<String>();

    if ciphered == expected {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}
