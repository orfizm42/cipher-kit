use super::Cipher;
use super::caesar::Caesar;

pub struct Rot13;

impl Cipher for Rot13 {
    fn name(&self) -> &str {
        "ROT13"
    }

    fn description(&self) -> &str {
        "アルファベットを13文字ずらす換字式暗号。2回適用で元に戻る。"
    }

    fn is_symmetric(&self) -> bool {
        true
    }

    fn encode(&self, input: &str) -> String {
        let c = Caesar { shift: 13 };
        c.encode(input)
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
        let r = Rot13;
        assert_eq!(r.encode(&r.encode("Hello World!")), "Hello World!");
    }

    #[test]
    fn test_empty() {
        let r = Rot13;
        assert_eq!(r.encode(""), "");
    }
}
