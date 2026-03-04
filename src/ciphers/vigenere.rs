use super::Cipher;

pub struct Vigenere {
    pub key: String,
}

impl Cipher for Vigenere {
    fn name(&self) -> &str {
        "ヴィジュネル暗号"
    }

    fn description(&self) -> &str {
        "キーワードを使って各文字のシフト量を変える多表式換字暗号。"
    }

    fn encode(&self, input: &str) -> String {
        let key_shifts: Vec<i32> = self
            .key
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase() as i32 - 'a' as i32)
            .collect();
        if key_shifts.is_empty() {
            return input.to_string();
        }
        let mut ki = 0;
        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let shift = key_shifts[ki % key_shifts.len()];
                    ki += 1;
                    if c.is_ascii_uppercase() {
                        ((c as u8 - b'A' + shift as u8) % 26 + b'A') as char
                    } else {
                        ((c as u8 - b'a' + shift as u8) % 26 + b'a') as char
                    }
                } else {
                    c
                }
            })
            .collect()
    }

    fn decode(&self, input: &str) -> String {
        let key_shifts: Vec<i32> = self
            .key
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase() as i32 - 'a' as i32)
            .collect();
        if key_shifts.is_empty() {
            return input.to_string();
        }
        let mut ki = 0;
        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let shift = key_shifts[ki % key_shifts.len()];
                    ki += 1;
                    if c.is_ascii_uppercase() {
                        ((c as u8 - b'A' + 26 - shift as u8) % 26 + b'A') as char
                    } else {
                        ((c as u8 - b'a' + 26 - shift as u8) % 26 + b'a') as char
                    }
                } else {
                    c
                }
            })
            .collect()
    }

    fn ui_params(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("キーワード:");
            ui.text_edit_singleline(&mut self.key);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ciphers::Cipher;

    #[test]
    fn test_known_vector() {
        let v = Vigenere {
            key: "LEMON".to_string(),
        };
        assert_eq!(v.encode("ATTACKATDAWN"), "LXFOPVEFRNHR");
        let v2 = Vigenere {
            key: "LEMON".to_string(),
        };
        assert_eq!(v2.decode("LXFOPVEFRNHR"), "ATTACKATDAWN");
    }

    #[test]
    fn test_empty_key() {
        let v = Vigenere {
            key: String::new(),
        };
        assert_eq!(v.encode("Hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        let v = Vigenere {
            key: "KEY".to_string(),
        };
        assert_eq!(v.encode(""), "");
    }
}
