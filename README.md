# gwt - Git Worktree Manager

> v2.0.0

A CLI tool for interactively creating, switching, and removing git worktrees.
Uses [Claude Code](https://github.com/anthropics/claude-code) for AI-powered branch name suggestions.

## Features

- **Arrow-key navigation** - Interactive menus with `↑↓`, `j/k` (vim), number keys, and `q` to cancel
- **AI branch naming** - Describe what you want to implement, get a branch name suggested by Claude
- **Direct create mode** - `gwt "implement auth"` skips the prompt and goes straight to branch selection
- **Multilingual** - 7 languages: English, Japanese, French, Spanish, Russian, Chinese, Arabic

## Requirements

- Bash 3.2+
- Git
- [Claude Code](https://github.com/anthropics/claude-code) (optional, for AI branch name suggestions)

## Installation

```bash
# 1. Copy the script to somewhere in your PATH
cp gwt /usr/local/bin/gwt
chmod +x /usr/local/bin/gwt

# 2. Copy the locale directory alongside the script
cp -r locale /usr/local/bin/locale

# 3. Add the shell wrapper to your .zshrc (required for gwt switch to cd)
echo 'source /path/to/gwt.zsh' >> ~/.zshrc
source ~/.zshrc
```

> **Note:** `gwt switch` uses a shell function to `cd` into the selected worktree. Without sourcing `gwt.zsh`, the directory change won't take effect in your current shell.

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `gwt create` | `c` | Create worktree (AI branch name suggestion) |
| `gwt list` | `l`, `ls` | List worktrees |
| `gwt switch` | `s` | Switch to worktree (interactive) |
| `gwt remove` | `rm` | Remove worktree (interactive) |
| `gwt help` | `-h` | Show help |

Running `gwt` with no arguments opens the interactive menu.

## Usage

```bash
# Interactive menu (arrow keys to navigate)
gwt

# Create a new worktree
gwt create

# Create directly from a description (skips the prompt)
gwt "implement user authentication"

# List all worktrees
gwt ls

# Switch to another worktree
gwt s

# Remove a worktree
gwt rm
```

### Keyboard shortcuts

All interactive menus support:

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `1`-`9` | Jump to item |
| `Enter` | Confirm selection |
| `q` | Cancel |

### Create workflow

1. Enter a description of what you want to implement (or pass it as an argument)
2. A branch name is suggested (AI-powered if Claude Code is installed, otherwise auto-generated)
3. Select the suggested name, enter your own, or regenerate with feedback
4. The worktree is created

## Language / Locale

gwt defaults to English. Set `GWT_LANG` or use your system `LANG` to switch languages:

```bash
# Use Japanese
GWT_LANG=ja gwt

# Or set it in your shell profile
export GWT_LANG=ja
```

### Supported languages

| Code | Language |
|------|----------|
| `en` | English (default) |
| `ja` | 日本語 |
| `fr` | Français |
| `es` | Español |
| `ru` | Русский |
| `zh` | 中文 |
| `ar` | العربية |

## How it works

Worktrees are created at `../<repo-name>-worktrees/<branch>/`:

```
parent/
├── my-project/              # Main worktree
└── my-project-worktrees/
    ├── feat-add-auth/       # Created by gwt
    └── fix-login-bug/       # Created by gwt
```

When Claude Code is installed, `gwt create` sends your implementation description to Claude to suggest a branch name (kebab-case with `feat/`, `fix/`, `refactor/` prefixes). Without Claude Code, it auto-generates from the description text.

## License

[MIT](LICENSE)
