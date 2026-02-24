# shuu - Git Worktree Manager

[日本語](README.ja.md)

> v0.0.1

A CLI tool for interactively creating, switching, and removing git worktrees.
Uses [Claude Code](https://github.com/anthropics/claude-code) for AI-powered branch name suggestions.

## Features

- **Arrow-key navigation** - Interactive menus with `↑↓`, `j/k` (vim), number keys, and `q` to cancel
- **AI branch naming** - Describe what you want to implement, get a branch name suggested by Claude
- **Direct create mode** - `shuu "implement auth"` skips the prompt and goes straight to branch selection
- **Multilingual** - 7 languages: English, Japanese, French, Spanish, Russian, Chinese, Arabic
- **First-run setup** - Language and AI model selection on first launch

## Requirements

- Git
- [Claude Code](https://github.com/anthropics/claude-code) (optional, for AI branch name suggestions)

## Installation

### One-liner install (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/kaye-dev/shuu-cli/main/install.sh | bash
```

Automatically detects your OS and architecture, downloads the latest release binary, and adds it to your PATH.

You can also specify a version:

```bash
curl -fsSL https://raw.githubusercontent.com/kaye-dev/shuu-cli/main/install.sh | bash -s v0.0.1
```

### Build from source

Requires [Rust](https://www.rust-lang.org/tools/install) toolchain.

```bash
git clone https://github.com/kaye-dev/shuu-cli.git
cd shuu-cli
cargo build --release
```

The binary is output to `target/release/shuu`. Copy it to a directory in your PATH:

```bash
cp target/release/shuu ~/.local/bin/
# or
sudo cp target/release/shuu /usr/local/bin/
```

### cargo install

```bash
cargo install --git https://github.com/kaye-dev/shuu-cli.git
```

This installs the `shuu` binary to `~/.cargo/bin/` (already in PATH if Rust is set up).

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `shuu` | | Interactive menu |
| `shuu create` | `c` | Create worktree (AI branch name suggestion) |
| `shuu list` | `l`, `ls` | List worktrees |
| `shuu switch` | `s` | Switch to worktree (interactive) |
| `shuu remove` | `rm` | Remove worktree (interactive) |
| `shuu settings` | | Configure language and AI model |
| `shuu help` | `-h` | Show help |

## Usage

```bash
# Interactive menu (arrow keys to navigate)
shuu

# Create a new worktree
shuu create

# Create directly from a description (skips the prompt)
shuu "implement user authentication"

# List all worktrees
shuu ls

# Switch to another worktree
shuu s

# Remove a worktree
shuu rm

# Open settings
shuu settings
```

### Keyboard shortcuts

All interactive menus support:

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `1`-`9` | Jump to item |
| `Enter` | Confirm selection |
| `q` / `Esc` | Cancel |

### Create workflow

1. Enter a description of what you want to implement (or pass it as an argument)
2. A branch name is suggested (AI-powered if Claude Code is installed, otherwise auto-generated)
3. Select the suggested name, enter your own, or regenerate with feedback
4. The worktree is created

## Language / Locale

shuu selects a language on first run. You can change it later via `shuu settings`, or override with `GWT_LANG`:

```bash
GWT_LANG=ja shuu
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
    ├── feat-add-auth/       # Created by shuu
    └── fix-login-bug/       # Created by shuu
```

When Claude Code is installed, `shuu create` sends your implementation description to Claude to suggest a branch name (kebab-case with `feat/`, `fix/`, `refactor/` prefixes). Without Claude Code, it auto-generates from the description text.

## License

[MIT](LICENSE)
