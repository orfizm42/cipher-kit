# cipher-kit

古典暗号のエンコード/デコードを GUI 上で手軽に行えるデスクトップアプリケーションです。

## なぜ作ったのか

暗号の歴史は紀元前にまで遡ります。シーザー暗号、ヴィジュネル暗号、上杉謙信が使ったとされる暗号――これらの古典暗号は、現代の暗号技術の原点です。

このアプリは、古典暗号の仕組みを **「自分の手で実装して、動かして理解する」** ことを目的に作りました。

- Rust の学習題材として、トレイトやモジュール分割を実践的に使う
- 暗号アルゴリズムを外部ライブラリに頼らず自前で書くことで、仕組みを深く理解する
- 日本独自の暗号（上杉暗号）にも対応し、和の暗号文化にも触れる

コードを読めば暗号の仕組みがそのまま分かる、という設計を意識しています。

## 対応暗号

| 暗号名 | 分類 | パラメータ | 特徴 |
|--------|------|-----------|------|
| シーザー暗号 | 換字式（単一） | シフト量 (1–25) | 最も基本的な暗号。ローマの将軍カエサルが使用 |
| ROT13 | 換字式（単一） | なし | シーザー暗号の特殊ケース。2回かけると元に戻る |
| アトバシュ暗号 | 換字式（単一） | なし | A↔Z のように反転。ヘブライ語聖書で使われた |
| ヴィジュネル暗号 | 換字式（多表） | キーワード | 16世紀に登場。300年間「解読不能」と呼ばれた |
| レールフェンス暗号 | 転置式 | レール数 (2–10) | 文字をジグザグに配置する転置暗号 |
| 上杉暗号 | 換字式（表引き） | なし | 上杉謙信が使ったとされる。いろは47文字の暗号表 |

## セットアップ

### 必要環境

- Rust 1.75 以上

### Linux の場合（egui の依存パッケージ）

```bash
# Ubuntu / Debian
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev libgtk-3-dev
```

### ビルド & 実行

```bash
git clone https://github.com/ORFIZM/cipher-kit.git
cd cipher-kit
cargo run --release
```

### テスト

```bash
cargo test
```

## 使い方

1. 左のサイドバーから暗号を選択
2. 必要に応じてパラメータ（シフト量、キーワードなど）を設定
3. 入力欄にテキストを入力
4. 「エンコード」または「デコード」ボタンを押す
5. 出力欄に結果が表示される

便利な機能として、「出力を入力へ」ボタンで変換結果をそのまま次の入力にできます。これにより、シーザー暗号をかけた後にさらにヴィジュネル暗号をかける、といった暗号の重ねがけも簡単に試せます。

## プロジェクト構成

```
cipher-kit/
├── Cargo.toml
└── src/
    ├── main.rs               # エントリーポイント + GUI
    └── ciphers/
        ├── mod.rs            # Cipher トレイト定義 + re-export
        ├── caesar.rs         # シーザー暗号
        ├── rot13.rs          # ROT13
        ├── atbash.rs         # アトバシュ暗号
        ├── vigenere.rs       # ヴィジュネル暗号
        ├── railfence.rs      # レールフェンス暗号
        └── uesugi.rs         # 上杉暗号
```

設計のポイントは、暗号 1 種 = 1 ファイルの分離と、全暗号が共通の `Cipher` トレイトを実装する構造です。GUI 側は `Cipher` トレイトだけに依存するため、新しい暗号を追加しても既存のコードを変更する必要がありません。

## 新しい暗号の追加方法

4 ステップで完了します。

### Step 1: `src/ciphers/` に新しいファイルを作る

例として「スキュタレー暗号」を追加する場合:

```rust
// src/ciphers/scytale.rs

use super::Cipher;

pub struct Scytale {
    pub columns: usize,
}

impl Cipher for Scytale {
    fn name(&self) -> &str {
        "スキュタレー暗号"
    }

    fn description(&self) -> &str {
        "古代ギリシャの転置式暗号。棒に革紐を巻きつけて書き、解くと文字が並び替わる。"
    }

    fn encode(&self, input: &str) -> String {
        // エンコード処理を実装
        todo!()
    }

    fn decode(&self, input: &str) -> String {
        // デコード処理を実装
        todo!()
    }

    fn ui_params(&mut self, ui: &mut egui::Ui) {
        // パラメータが必要ならスライダーなどを追加
        ui.add(egui::Slider::new(&mut self.columns, 2..=20).text("列数"));
    }
}
```

`Cipher` トレイトのメソッドについて:

| メソッド | 必須? | 説明 |
|---------|-------|------|
| `name()` | 必須 | サイドバーに表示される暗号名 |
| `description()` | 必須 | サイドバー下部に表示される説明文 |
| `encode()` | 必須 | エンコード処理 |
| `decode()` | 必須 | デコード処理 |
| `is_symmetric()` | 任意 | `true` にするとモード選択が非表示になる（デフォルト: `false`） |
| `ui_params()` | 任意 | スライダーやテキスト入力など、パラメータ UI を描画 |
| `ui_extra()` | 任意 | 暗号固有の追加 UI（上杉暗号の暗号表など） |

### Step 2: `src/ciphers/mod.rs` にモジュールを追加

```rust
pub mod scytale;  // この1行を追加
```

### Step 3: `src/main.rs` の暗号リストに登録

```rust
impl Default for CipherApp {
    fn default() -> Self {
        Self {
            ciphers: vec![
                // ...既存の暗号...
                Box::new(Scytale { columns: 5 }),  // この1行を追加
            ],
            // ...
        }
    }
}
```

### Step 4: テストを書く

```rust
// src/ciphers/scytale.rs の末尾に追加

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let cipher = Scytale { columns: 5 };
        let original = "HELLO WORLD";
        let encoded = cipher.encode(original);
        let decoded = cipher.decode(&encoded);
        assert_eq!(decoded, original);
    }
}
```

以上で完了です。既存のファイルへの変更は `mod.rs` に 1 行、`main.rs` に 1 行のみ。

## 技術スタック

| 技術 | 用途 |
|------|------|
| Rust | アプリケーション言語 |
| egui / eframe 0.31 | GUI フレームワーク（即時モード） |
| Noto Sans CJK | 日本語フォント（上杉暗号用、システムフォントを自動検出） |

全アルゴリズムを自前で実装しており、外部の暗号ライブラリには依存していません。

## 作者

**ORFIZM**

## ライセンス

MIT License — 詳細は [LICENSE](LICENSE) を参照してください。

## 今後の拡張候補

- ポリュビオスの暗号
- プレイフェア暗号
- エニグマ風暗号機シミュレーション
- 頻度分析ツール（暗号解読の補助）
- ダークモード / テーマ切り替え
- 暗号の仕組みを視覚的に示すアニメーション
