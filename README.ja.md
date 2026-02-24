# shuu - Git Worktree マネージャー

> v0.0.1

Git worktree をインタラクティブに作成・切替・削除する CLI ツール。
[Claude Code](https://github.com/anthropics/claude-code) を利用した AI ブランチ名提案機能付き。

## 特徴

- **矢印キーナビゲーション** - `↑↓`、`j/k` (vim)、数字キー、`q` でキャンセル
- **AI ブランチ名提案** - 実装内容を入力すると Claude がブランチ名を提案
- **ダイレクト作成** - `shuu "認証機能の実装"` でプロンプトをスキップ
- **多言語対応** - 英語、日本語、フランス語、スペイン語、ロシア語、中国語、アラビア語
- **初回セットアップ** - 初回起動時に言語と AI モデルを選択

## 必要要件

- Git
- [Claude Code](https://github.com/anthropics/claude-code)（任意、AI ブランチ名提案に使用）

## インストール

### ワンライナーインストール（推奨）

```bash
curl -fsSL https://raw.githubusercontent.com/kaye-dev/shuu-cli/main/install.sh | bash
```

OS とアーキテクチャを自動検出し、最新リリースバイナリをダウンロードして PATH に追加します。

バージョンを指定する場合:

```bash
curl -fsSL https://raw.githubusercontent.com/kaye-dev/shuu-cli/main/install.sh | bash -s v0.0.1
```

### ソースからビルド

[Rust](https://www.rust-lang.org/tools/install) ツールチェーンが必要です。

```bash
git clone https://github.com/kaye-dev/shuu-cli.git
cd shuu-cli
cargo build --release
```

バイナリは `target/release/shuu` に出力されます。PATH の通ったディレクトリにコピーしてください:

```bash
cp target/release/shuu ~/.local/bin/
# または
sudo cp target/release/shuu /usr/local/bin/
```

### cargo install

```bash
cargo install --git https://github.com/kaye-dev/shuu-cli.git
```

`~/.cargo/bin/` に `shuu` バイナリがインストールされます（Rust セットアップ済みなら PATH に含まれています）。

## コマンド

| コマンド | エイリアス | 説明 |
|---------|-----------|------|
| `shuu` | | インタラクティブメニュー |
| `shuu create` | `c` | worktree 作成（AI ブランチ名提案） |
| `shuu list` | `l`, `ls` | worktree 一覧 |
| `shuu switch` | `s` | worktree 切替（インタラクティブ） |
| `shuu remove` | `rm` | worktree 削除（インタラクティブ） |
| `shuu settings` | | 言語・AI モデル設定 |
| `shuu help` | `-h` | ヘルプ表示 |

## 使い方

```bash
# インタラクティブメニュー
shuu

# worktree を作成
shuu create

# 説明文から直接作成（プロンプトをスキップ）
shuu "ユーザー認証の実装"

# worktree 一覧
shuu ls

# worktree を切替
shuu s

# worktree を削除
shuu rm

# 設定
shuu settings
```

### キーボードショートカット

すべてのインタラクティブメニューで使用可能:

| キー | 操作 |
|-----|------|
| `↑` / `k` | 上に移動 |
| `↓` / `j` | 下に移動 |
| `1`-`9` | 項目にジャンプ |
| `Enter` | 選択確定 |
| `q` / `Esc` | キャンセル |

### 作成ワークフロー

1. 実装内容を入力（または引数で渡す）
2. ブランチ名が提案される（Claude Code がインストール済みなら AI、なければ自動生成）
3. 提案を選択、手入力、またはフィードバック付きで再生成
4. worktree が作成される

## 言語設定

初回起動時に言語を選択します。後から `shuu settings` で変更可能です。環境変数で上書きもできます:

```bash
GWT_LANG=ja shuu
```

### 対応言語

| コード | 言語 |
|-------|------|
| `en` | English（デフォルト） |
| `ja` | 日本語 |
| `fr` | Français |
| `es` | Español |
| `ru` | Русский |
| `zh` | 中文 |
| `ar` | العربية |

## 仕組み

worktree は `../<リポジトリ名>-worktrees/<ブランチ>/` に作成されます:

```
parent/
├── my-project/              # メイン worktree
└── my-project-worktrees/
    ├── feat-add-auth/       # shuu で作成
    └── fix-login-bug/       # shuu で作成
```

Claude Code がインストールされている場合、`shuu create` は実装内容の説明を Claude に送り、ブランチ名を提案させます（`feat/`、`fix/`、`refactor/` などのプレフィックス付きケバブケース）。Claude Code がない場合は説明文から自動生成します。

## ライセンス

[MIT](LICENSE)
