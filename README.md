# gwt - Git Worktree Manager

> v1.0.0

Git worktree の作成・切り替え・削除を対話的に行う CLI ツールです。
ブランチ名の提案に [Claude Code](https://github.com/anthropics/claude-code) を活用できます。

## 要件

- Bash 3.2+
- Git
- [Claude Code](https://github.com/anthropics/claude-code)（オプション、ブランチ名の AI 提案に使用）

## インストール

```bash
# 1. スクリプトを PATH の通った場所にコピー
cp gwt /usr/local/bin/gwt
chmod +x /usr/local/bin/gwt

# 2. シェルラッパーを .zshrc に追加（gwt switch での cd に必要）
echo 'source /path/to/gwt.zsh' >> ~/.zshrc
source ~/.zshrc
```

> **Note:** `gwt switch` はサブプロセスからの `cd` をシェル関数で実現しています。`gwt.zsh` を source しないと、switch 後にディレクトリが移動しません。

## コマンド

| コマンド | 短縮 | 説明 |
|----------|------|------|
| `gwt create` | `c` | worktree を作成（AI ブランチ名提案付き） |
| `gwt list` | `l`, `ls` | worktree 一覧を表示 |
| `gwt switch` | `s` | worktree へ移動（対話式選択） |
| `gwt remove` | `rm` | worktree を削除（対話式選択） |
| `gwt help` | `-h` | ヘルプを表示 |

引数なしで `gwt` を実行するとインタラクティブメニューが表示されます。

## 使い方

```bash
# インタラクティブメニューを起動
gwt

# 新しい worktree を作成
gwt create

# 一覧を確認
gwt ls

# 別の worktree に移動
gwt s

# 不要な worktree を削除
gwt rm
```

### create の流れ

1. 実装内容の説明を入力
2. ブランチ名が提案される（Claude Code がある場合は AI 提案、なければ自動生成）
3. ブランチ名を確認・修正して Enter
4. worktree が作成される

## 仕組み

worktree は `../<リポジトリ名>-worktrees/<ブランチ名>/` に作成されます。

```
parent/
├── my-project/              # メインの worktree
└── my-project-worktrees/
    ├── feat-add-auth/        # gwt create で作成
    └── fix-login-bug/        # gwt create で作成
```

`gwt create` 実行時に Claude Code がインストールされていれば、実装内容の説明からブランチ名を自動提案します（`feat/`, `fix/`, `refactor/` 等のプレフィックス付き kebab-case）。インストールされていない場合は説明文から `feat/` プレフィックス付きで自動生成します。
