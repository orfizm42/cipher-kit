pub trait Cipher {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn encode(&self, input: &str) -> String;
    fn decode(&self, input: &str) -> String;
    fn is_symmetric(&self) -> bool {
        false
    }
    fn ui_params(&mut self, _ui: &mut egui::Ui) {}
    fn ui_extra(&mut self, _ui: &mut egui::Ui) {}
}

pub mod caesar;
pub mod rot13;
pub mod atbash;
pub mod vigenere;
pub mod railfence;
pub mod uesugi;

pub use caesar::Caesar;
pub use rot13::Rot13;
pub use atbash::Atbash;
pub use vigenere::Vigenere;
pub use railfence::RailFence;
pub use uesugi::Uesugi;
