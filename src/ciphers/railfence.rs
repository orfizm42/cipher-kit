use super::Cipher;

pub struct RailFence {
    pub rails: usize,
}

impl Cipher for RailFence {
    fn name(&self) -> &str {
        "レールフェンス暗号"
    }

    fn description(&self) -> &str {
        "文字をジグザグ状に配置して読み取る転置式暗号。"
    }

    fn encode(&self, input: &str) -> String {
        if self.rails <= 1 || self.rails >= input.len() {
            return input.to_string();
        }
        let chars: Vec<char> = input.chars().collect();
        let mut fence: Vec<Vec<char>> = vec![vec![]; self.rails];
        let mut rail = 0usize;
        let mut direction: i32 = 1;
        for &c in &chars {
            fence[rail].push(c);
            if rail == 0 {
                direction = 1;
            } else if rail == self.rails - 1 {
                direction = -1;
            }
            rail = (rail as i32 + direction) as usize;
        }
        fence.into_iter().flatten().collect()
    }

    fn decode(&self, input: &str) -> String {
        if self.rails <= 1 || self.rails >= input.len() {
            return input.to_string();
        }
        let len = input.len();
        let chars: Vec<char> = input.chars().collect();
        // Calculate the length of each rail
        let mut rail_lens = vec![0usize; self.rails];
        let mut rail = 0usize;
        let mut direction: i32 = 1;
        for _ in 0..len {
            rail_lens[rail] += 1;
            if rail == 0 {
                direction = 1;
            } else if rail == self.rails - 1 {
                direction = -1;
            }
            rail = (rail as i32 + direction) as usize;
        }
        // Split the ciphertext into rails
        let mut rail_chars: Vec<Vec<char>> = Vec::new();
        let mut idx = 0;
        for &rlen in &rail_lens {
            rail_chars.push(chars[idx..idx + rlen].to_vec());
            idx += rlen;
        }
        // Read off in zigzag order
        let mut rail_idx = vec![0usize; self.rails];
        let mut result = Vec::with_capacity(len);
        let mut r = 0usize;
        let mut dir: i32 = 1;
        for _ in 0..len {
            result.push(rail_chars[r][rail_idx[r]]);
            rail_idx[r] += 1;
            if r == 0 {
                dir = 1;
            } else if r == self.rails - 1 {
                dir = -1;
            }
            r = (r as i32 + dir) as usize;
        }
        result.into_iter().collect()
    }

    fn ui_params(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.rails, 2..=10).text("レール数"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ciphers::Cipher;

    #[test]
    fn test_encode_decode() {
        let r = RailFence { rails: 3 };
        let encoded = r.encode("Hello World!");
        let r2 = RailFence { rails: 3 };
        assert_eq!(r2.decode(&encoded), "Hello World!");
    }

    #[test]
    fn test_single_rail() {
        let r = RailFence { rails: 1 };
        assert_eq!(r.encode("Hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        let r = RailFence { rails: 3 };
        assert_eq!(r.encode(""), "");
    }
}
