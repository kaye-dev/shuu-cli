# shuu

[English](README.md)

Git worktree をインタラクティブに管理する CLI ツール。[Claude Code](https://github.com/anthropics/claude-code) による AI ブランチ名提案機能付き。

## インストール

```bash
curl -fsSL https://raw.githubusercontent.com/kaye-dev/shuu-cli/main/install.sh | bash
```

ソースからビルドする場合:

```bash
git clone https://github.com/kaye-dev/shuu-cli.git && cd shuu-cli
cargo build --release
cp target/release/shuu ~/.local/bin/
```

## 使い方

```bash
shuu                          # インタラクティブメニュー
shuu create                   # worktree 作成
shuu "認証機能の実装"            # AI ブランチ名で作成
shuu list                     # worktree 一覧
shuu switch                   # worktree 切替
shuu remove                   # worktree 削除
shuu settings                 # 言語・AI モデル設定
```

## ライセンス

[MIT](LICENSE)
