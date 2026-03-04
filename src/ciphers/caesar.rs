use super::Cipher;

pub struct Caesar {
    pub shift: i32,
}

impl Cipher for Caesar {
    fn name(&self) -> &str {
        "シーザー暗号"
    }

    fn description(&self) -> &str {
        "各文字をアルファベット上で指定した数だけずらす換字式暗号。"
    }

    fn encode(&self, input: &str) -> String {
        let shift = ((self.shift % 26) + 26) % 26;
        input
            .chars()
            .map(|c| {
                if c.is_ascii_uppercase() {
                    let base = b'A';
                    (((c as u8 - base) as i32 + shift) % 26 + base as i32) as u8 as char
                } else if c.is_ascii_lowercase() {
                    let base = b'a';
                    (((c as u8 - base) as i32 + shift) % 26 + base as i32) as u8 as char
                } else {
                    c
                }
            })
            .collect()
    }

    fn decode(&self, input: &str) -> String {
        let decode_shift = ((26 - self.shift % 26) % 26 + 26) % 26;
        let decoder = Caesar { shift: decode_shift };
        decoder.encode(input)
    }

    fn ui_params(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.shift, 1..=25).text("シフト量"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ciphers::Cipher;

    #[test]
    fn test_encode_decode() {
        let c = Caesar { shift: 3 };
        let encoded = c.encode("Hello World!");
        let c2 = Caesar { shift: 3 };
        assert_eq!(c2.decode(&encoded), "Hello World!");
    }

    #[test]
    fn test_non_alpha() {
        let c = Caesar { shift: 3 };
        assert_eq!(c.encode("Hello 123!"), "Khoor 123!");
    }

    #[test]
    fn test_empty() {
        let c = Caesar { shift: 3 };
        assert_eq!(c.encode(""), "");
    }
}
