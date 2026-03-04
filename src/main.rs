mod ciphers;

use ciphers::{Cipher, Caesar, Rot13, Atbash, Vigenere, RailFence, Uesugi};
use eframe::egui;
use egui::{Color32, Margin, Stroke};
use egui::epaint::CornerRadius;

// ── 暗号カテゴリ（サイドバー表示用、インデックスは ciphers vec に対応）───
const CIPHER_CATEGORIES: [&str; 6] = [
    "換字式（単一）",
    "換字式（単一）",
    "換字式（単一）",
    "換字式（多表）",
    "転置式",
    "換字式（表引き）",
];

// ── カラーパレット ──────────────────────────────────────────────
const BG: Color32             = Color32::from_rgb(248, 249, 250);
const SIDEBAR_BG: Color32     = Color32::from_rgb(237, 241, 247);
const ACCENT: Color32         = Color32::from_rgb(59, 130, 246);
const TEXT_DARK: Color32      = Color32::from_rgb(33,  37,  41);
const TEXT_MUTED: Color32     = Color32::from_rgb(107, 114, 128);
const WIDGET_BG: Color32      = Color32::from_rgb(226, 232, 240);
const WIDGET_HOVER: Color32   = Color32::from_rgb(203, 213, 225);
const SELECTED_BG: Color32    = Color32::from_rgb(219, 234, 254);

fn setup_visuals(ctx: &egui::Context) {
    let mut v = egui::Visuals::light();

    v.panel_fill    = BG;
    v.window_fill   = BG;
    v.window_corner_radius = CornerRadius::same(8);
    v.override_text_color  = Some(TEXT_DARK);

    v.selection.bg_fill = SELECTED_BG;
    v.selection.stroke  = Stroke::new(1.0, ACCENT);

    let r = CornerRadius::same(6);

    v.widgets.noninteractive.corner_radius = r;
    v.widgets.noninteractive.bg_fill       = BG;
    v.widgets.noninteractive.weak_bg_fill  = BG;
    v.widgets.noninteractive.fg_stroke     = Stroke::new(1.0, TEXT_MUTED);
    v.widgets.noninteractive.bg_stroke     = Stroke::new(1.0, Color32::from_rgb(213, 219, 228));

    v.widgets.inactive.corner_radius = r;
    v.widgets.inactive.bg_fill       = WIDGET_BG;
    v.widgets.inactive.weak_bg_fill  = WIDGET_BG;
    v.widgets.inactive.fg_stroke     = Stroke::new(1.0, TEXT_DARK);
    v.widgets.inactive.bg_stroke     = Stroke::NONE;

    v.widgets.hovered.corner_radius = r;
    v.widgets.hovered.bg_fill       = WIDGET_HOVER;
    v.widgets.hovered.weak_bg_fill  = WIDGET_HOVER;
    v.widgets.hovered.fg_stroke     = Stroke::new(1.5, TEXT_DARK);
    v.widgets.hovered.bg_stroke     = Stroke::new(1.0, ACCENT);

    v.widgets.active.corner_radius = r;
    v.widgets.active.bg_fill       = ACCENT;
    v.widgets.active.weak_bg_fill  = ACCENT;
    v.widgets.active.fg_stroke     = Stroke::new(1.5, Color32::WHITE);
    v.widgets.active.bg_stroke     = Stroke::NONE;

    v.widgets.open.corner_radius = r;
    v.widgets.open.bg_fill       = WIDGET_BG;
    v.widgets.open.weak_bg_fill  = WIDGET_BG;
    v.widgets.open.fg_stroke     = Stroke::new(1.0, TEXT_DARK);

    ctx.set_visuals(v);
}

// ── アプリ状態 ──────────────────────────────────────────────────
#[derive(PartialEq)]
enum Mode {
    Encode,
    Decode,
}

struct CipherApp {
    ciphers:  Vec<Box<dyn Cipher>>,
    selected: usize,
    mode:     Mode,
    input:    String,
    output:   String,
}

impl Default for CipherApp {
    fn default() -> Self {
        Self {
            ciphers: vec![
                Box::new(Caesar  { shift: 3 }),
                Box::new(Rot13),
                Box::new(Atbash),
                Box::new(Vigenere { key: String::new() }),
                Box::new(RailFence { rails: 3 }),
                Box::new(Uesugi  { show_table: false }),
            ],
            selected: 0,
            mode:   Mode::Encode,
            input:  String::new(),
            output: String::new(),
        }
    }
}

impl eframe::App for CipherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── サイドバー ────────────────────────────────────────────
        egui::SidePanel::left("cipher_selector")
            .exact_width(180.0)
            .frame(
                egui::Frame::NONE
                    .fill(SIDEBAR_BG)
                    .inner_margin(Margin::symmetric(12, 10)),
            )
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("🔐 暗号の種類")
                        .strong()
                        .color(TEXT_DARK),
                );
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);

                for (i, cipher) in self.ciphers.iter().enumerate() {
                    let selected = self.selected == i;
                    let bg = if selected { SELECTED_BG } else { Color32::TRANSPARENT };

                    let resp = egui::Frame::NONE
                        .fill(bg)
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::symmetric(8, 5))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            let name_color = if selected { ACCENT } else { TEXT_DARK };
                            ui.label(
                                egui::RichText::new(cipher.name())
                                    .strong()
                                    .color(name_color),
                            );
                            ui.label(
                                egui::RichText::new(CIPHER_CATEGORIES[i])
                                    .small()
                                    .color(TEXT_MUTED),
                            );
                        })
                        .response
                        .interact(egui::Sense::click());

                    if resp.clicked() {
                        self.selected = i;
                        self.output.clear();
                    }
                    // ホバー時に薄いハイライト
                    if resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    ui.add_space(2.0);
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.label(
                    egui::RichText::new("ℹ 説明")
                        .small()
                        .strong()
                        .color(TEXT_MUTED),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(self.ciphers[self.selected].description())
                        .small()
                        .color(TEXT_DARK),
                );
            });

        // ── メインパネル ──────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);

            ui.heading(self.ciphers[self.selected].name());
            ui.add_space(2.0);
            ui.separator();
            ui.add_space(10.0);

            let is_symmetric = self.ciphers[self.selected].is_symmetric();

            // モード選択（対称暗号は非表示）
            if !is_symmetric {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("モード:").color(TEXT_MUTED));
                    ui.radio_value(&mut self.mode, Mode::Encode, "🔒 エンコード");
                    ui.radio_value(&mut self.mode, Mode::Decode, "🔓 デコード");
                });
                ui.add_space(6.0);
            }

            // 暗号固有パラメータ UI
            self.ciphers[self.selected].ui_params(ui);

            // 追加 UI（上杉暗号表など）
            self.ciphers[self.selected].ui_extra(ui);

            ui.add_space(10.0);

            // 入力ラベル（モードに応じて変化）
            let input_label = if is_symmetric {
                "テキストを入力"
            } else if self.mode == Mode::Encode {
                "平文を入力"
            } else {
                "暗号文を入力"
            };
            ui.label(egui::RichText::new(input_label).small().color(TEXT_MUTED));
            ui.add_space(2.0);
            ui.add(
                egui::TextEdit::multiline(&mut self.input)
                    .desired_rows(5)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace),
            );

            ui.add_space(10.0);

            // 操作ボタン
            let btn_label = if is_symmetric {
                "🔄 変換"
            } else if self.mode == Mode::Encode {
                "🔒 エンコード"
            } else {
                "🔓 デコード"
            };

            ui.horizontal(|ui| {
                // プライマリボタン（アクセントカラー）
                let primary = egui::Button::new(
                    egui::RichText::new(btn_label).color(Color32::WHITE).strong(),
                )
                .fill(ACCENT);
                if ui
                    .add(primary)
                    .on_hover_text("入力テキストを変換します")
                    .clicked()
                {
                    let cipher = &self.ciphers[self.selected];
                    self.output = if is_symmetric || self.mode == Mode::Encode {
                        cipher.encode(&self.input)
                    } else {
                        cipher.decode(&self.input)
                    };
                }

                if ui
                    .button("🗑 クリア")
                    .on_hover_text("入力・出力を両方クリアします")
                    .clicked()
                {
                    self.input.clear();
                    self.output.clear();
                }

                if !self.output.is_empty() {
                    if ui
                        .button("📋 コピー")
                        .on_hover_text("出力をクリップボードにコピーします")
                        .clicked()
                    {
                        ctx.copy_text(self.output.clone());
                    }
                    if ui
                        .button("⬇ 出力を入力へ")
                        .on_hover_text("出力を入力欄に移します（暗号の連鎖適用に便利）")
                        .clicked()
                    {
                        self.input = std::mem::take(&mut self.output);
                    }
                }
            });

            ui.add_space(10.0);

            // 出力ラベル（モードに応じて変化）
            let output_label = if is_symmetric {
                "変換結果"
            } else if self.mode == Mode::Encode {
                "暗号文（出力）"
            } else {
                "平文（出力）"
            };
            ui.label(egui::RichText::new(output_label).small().color(TEXT_MUTED));
            ui.add_space(2.0);
            ui.add(
                egui::TextEdit::multiline(&mut self.output)
                    .desired_rows(5)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace),
            );
        });
    }
}

// ── フォント読み込み（バイナリ同梱）────────────────────────────
fn load_cjk_font() -> Option<Vec<u8>> {
    Some(include_bytes!("../assets/NotoSansJP.ttf").to_vec())
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "古典暗号ツール",
        native_options,
        Box::new(|cc| {
            setup_visuals(&cc.egui_ctx);

            if let Some(font_data) = load_cjk_font() {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "noto_cjk".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "noto_cjk".to_owned());
                fonts
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .push("noto_cjk".to_owned());
                cc.egui_ctx.set_fonts(fonts);
            }

            Ok(Box::new(CipherApp::default()))
        }),
    )
}
