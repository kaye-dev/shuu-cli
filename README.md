# shuu

[日本語](README.ja.md)

Interactive Git worktree manager with AI-powered branch name suggestions via [Claude Code](https://github.com/anthropics/claude-code).

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/kaye-dev/shuu-cli/main/install.sh | bash
```

Or build from source:

```bash
git clone https://github.com/kaye-dev/shuu-cli.git && cd shuu-cli
cargo build --release
cp target/release/shuu ~/.local/bin/
```

## Usage

```bash
shuu                          # Interactive menu
shuu create                   # Create worktree
shuu "implement auth"         # Create with AI branch name
shuu list                     # List worktrees
shuu switch                   # Switch worktree
shuu remove                   # Remove worktree
shuu settings                 # Language & AI model
```

## License

[MIT](LICENSE)
