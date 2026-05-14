# Rust Metronome with Log 🎵

Rustで実装された、練習ログ保存機能付きのメトロノームアプリです。
Python (Tkinter) 版をベースに、より高い精度とモダンなUI（eframe/egui）で再構築されました。
<img width="433" height="536" alt="スクリーンショット 2026-05-14 164923" src="https://github.com/user-attachments/assets/4168fa66-ef70-444c-8a6a-2beab8f5edb9" />

## 主な機能

- **高精度メトロノーム**: 40 BPM から 240 BPM まで対応。
- **拍子設定**: 2, 3, 4, 6 拍子を選択可能。
- **練習ログ機能**: 練習を停止するたびに、日付・BPM・練習時間を `practice_log.txt` に自動保存。
- **モダンなUI**: `egui` を使用した、シンプルで軽量なインターフェース。
- **クロスプラットフォーム**: Windows, macOS, Linux で動作（Rust環境が必要）。

## インストールと実行方法

### 1. 事前準備
Rustのビルド環境が必要です。インストールされていない場合は [rustup.rs](https://rustup.rs/) からインストールしてください。

### 2. コンパイルと実行
リポジトリをクローンするか、ファイルをダウンロードして以下のコマンドを実行します。

```bash
cargo run --release
使用しているライブラリ
eframe / egui - GUIフレームワーク

rodio - オーディオ再生

chrono - 日時管理

ログファイルの形式
練習を停止すると、プロジェクトフォルダ内に practice_log.txt が生成されます。
形式は以下の通りです：
[2024-05-14 17:30:45] BPM:120, 拍子:4, 時間:45秒

ライセンス
このプロジェクトは MIT License の下で公開されています。
