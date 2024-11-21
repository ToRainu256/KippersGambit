# KippersGambit

## 概要

**KippersGambit**は、合法的なチェスの手を返すシンプルなチェスエンジンです。**UCIに準拠**しています。

## 動作環境

-  Rust 1.82

## インストール

```bash
git clone https://github.com/yourusername/KippersGambit.git
cd KippersGambit
cargo build --release
```

## 使い方
### 基本的な使い方:
```bash
./target/release/KippersGambit
```
エンジンは現在のボード状態から可能な合法手を出力します。

### UCI対応のGUIで使用

**Bankisiaでの使用(推奨)**
KippersGambitはBankisiaでの動作確認が取れています。

1. Bankisiaのインストール
2. Bankisia上でKippersGambitを指定

## ToDo

* 評価関数の実装: ボードの状態を評価するアルゴリズムを追加
* 最善手の選択: 最適な手を選ぶロジックの導入
* アルファベータ剪定: 探索効率を向上させるアルゴリズムの実装
* テストの追加: ユニットテストの整備
* パフォーマンスの最適化(優先度低): コードの高速化と最適化

このREADMEはChatGPTによって作成されました。
