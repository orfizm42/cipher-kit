use super::Cipher;

const IROHA: [char; 48] = [
    'い', 'ろ', 'は', 'に', 'ほ', 'へ', 'と',
    'ち', 'り', 'ぬ', 'る', 'を', 'わ', 'か',
    'よ', 'た', 'れ', 'そ', 'つ', 'ね', 'な',
    'ら', 'む', 'う', 'ゐ', 'の', 'お', 'く',
    'や', 'ま', 'け', 'ふ', 'こ', 'え', 'て',
    'あ', 'さ', 'き', 'ゆ', 'め', 'み', 'し',
    'ゑ', 'ひ', 'も', 'せ', 'す', 'ん',
];

pub struct Uesugi {
    pub show_table: bool,
}

impl Cipher for Uesugi {
    fn name(&self) -> &str {
        "上杉暗号"
    }

    fn description(&self) -> &str {
        "いろは47文字を7x7の表に配置し、行列番号で表す換字式暗号。"
    }

    fn encode(&self, input: &str) -> String {
        let mut parts: Vec<String> = Vec::new();
        for c in input.chars() {
            if let Some(pos) = IROHA.iter().position(|&ic| ic == c) {
                let row = pos / 7 + 1;
                let col = pos % 7 + 1;
                parts.push(format!("{}{}", row, col));
            } else {
                parts.push(c.to_string());
            }
        }
        parts.join(" ")
    }

    fn decode(&self, input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }
        let mut result = String::new();
        for token in input.split_whitespace() {
            if token.len() == 2 && token.chars().all(|c| c.is_ascii_digit()) {
                let row: usize = token[0..1].parse().unwrap();
                let col: usize = token[1..2].parse().unwrap();
                if row >= 1 && row <= 7 && col >= 1 && col <= 7 && !(row == 7 && col == 7) {
                    let idx = (row - 1) * 7 + (col - 1);
                    if idx < IROHA.len() {
                        result.push(IROHA[idx]);
                    } else {
                        result.push_str(token);
                    }
                } else {
                    result.push_str(token);
                }
            } else {
                result.push_str(token);
            }
        }
        result
    }

    fn ui_extra(&mut self, ui: &mut egui::Ui) {
        ui.toggle_value(&mut self.show_table, "暗号表を表示");
        if self.show_table {
            egui::Grid::new("uesugi_table").show(ui, |ui| {
                // Header row
                ui.label("");
                for col in 1..=7 {
                    ui.label(col.to_string());
                }
                ui.end_row();
                // Data rows
                for row in 0..7 {
                    ui.label((row + 1).to_string());
                    for col in 0..7 {
                        let idx = row * 7 + col;
                        if idx < IROHA.len() {
                            ui.label(IROHA[idx].to_string());
                        } else {
                            ui.label("—");
                        }
                    }
                    ui.end_row();
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ciphers::Cipher;

    #[test]
    fn test_encode_decode() {
        let u = Uesugi { show_table: false };
        let encoded = u.encode("いろはにほへと");
        let u2 = Uesugi { show_table: false };
        assert_eq!(u2.decode(&encoded), "いろはにほへと");
    }

    #[test]
    fn test_non_hiragana() {
        let u = Uesugi { show_table: false };
        assert_eq!(u.encode("Hello"), "H e l l o");
    }

    #[test]
    fn test_empty() {
        let u = Uesugi { show_table: false };
        assert_eq!(u.encode(""), "");
    }
}
