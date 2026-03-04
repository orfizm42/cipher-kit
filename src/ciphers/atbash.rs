use super::Cipher;

pub struct Atbash;

impl Cipher for Atbash {
    fn name(&self) -> &str {
        "アトバシュ暗号"
    }

    fn description(&self) -> &str {
        "AとZ、BとYのように文字を反転させるヘブライ語起源の暗号。2回適用で元に戻る。"
    }

    fn is_symmetric(&self) -> bool {
        true
    }

    fn encode(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| {
                if c.is_ascii_uppercase() {
                    (b'Z' - (c as u8 - b'A')) as char
                } else if c.is_ascii_lowercase() {
                    (b'z' - (c as u8 - b'a')) as char
                } else {
                    c
                }
            })
            .collect()
    }

    fn decode(&self, input: &str) -> String {
        self.encode(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ciphers::Cipher;

    #[test]
    fn test_symmetric() {
        let a = Atbash;
        assert_eq!(a.encode(&a.encode("Hello World!")), "Hello World!");
    }

    #[test]
    fn test_empty() {
        let a = Atbash;
        assert_eq!(a.encode(""), "");
    }
}
