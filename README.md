# py-rust-tsp

python から Rust製巡回セールスマン問題ソルバ を呼び出すサンプル

# 本モジュールのサンプル実行のための環境設定

1. rustのセットアップ。使用するRustのバージョンは1.66.0.

1. pyenv, poetry をセットアップする

1. `poetry install` を実行してpython部分実行に必要なモジュールをセットアップする

1. `poetry run python setup.py install` を実行して python向けRustモジュールをビルドする

1. `notebook/sample.ipynb` を開き、セルを順に実行して、エラーが出なければOK.成功すれば計算結果が表示される


# 開発用コマンド呼び出し方(メモ)

- Rustパートのテスト実行

```
cargo run --bin py-rust-tsp
```
