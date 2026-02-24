#!/usr/bin/env bash
set -euo pipefail

REPO="kaye-dev/shuu-cli"
INSTALL_DIR="${SHUU_INSTALL_DIR:-$HOME/.shuu/bin}"

# ── Helpers ──────────────────────────────────────────────────────────

info()    { printf '\033[0;36m▸\033[0m %s\n' "$1"; }
success() { printf '\033[0;32m✓\033[0m %s\n' "$1"; }
err()     { printf '\033[0;31merror:\033[0m %s\n' "$1" >&2; exit 1; }

# ── Detect OS ────────────────────────────────────────────────────────

detect_os() {
  case "$(uname -s)" in
    Darwin*) echo "darwin" ;;
    Linux*)  echo "linux" ;;
    *)       err "Unsupported OS: $(uname -s). Only macOS and Linux are supported." ;;
  esac
}

# ── Detect Architecture ─────────────────────────────────────────────

detect_arch() {
  local arch
  arch="$(uname -m)"
  case "$arch" in
    x86_64|amd64)  echo "x64" ;;
    aarch64|arm64) echo "arm64" ;;
    *)             err "Unsupported architecture: $arch" ;;
  esac
}

# ── Detect shell config file ────────────────────────────────────────

detect_shell_config() {
  local shell_name
  shell_name="$(basename "${SHELL:-/bin/sh}")"
  case "$shell_name" in
    zsh)  echo "${ZDOTDIR:-$HOME}/.zshrc" ;;
    bash)
      if [ -f "$HOME/.bashrc" ]; then
        echo "$HOME/.bashrc"
      else
        echo "$HOME/.bash_profile"
      fi
      ;;
    fish) echo "${XDG_CONFIG_HOME:-$HOME/.config}/fish/config.fish" ;;
    *)    echo "$HOME/.profile" ;;
  esac
}

# ── Main ─────────────────────────────────────────────────────────────

main() {
  local os arch version filename url

  os="$(detect_os)"
  arch="$(detect_arch)"

  info "Detected platform: ${os}-${arch}"

  # Resolve version
  if [ -n "${1:-}" ]; then
    version="$1"
  else
    info "Fetching latest version..."
    version="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
      | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"//;s/".*//')"
    if [ -z "$version" ]; then
      err "Failed to fetch latest version. Check your network connection."
    fi
  fi

  info "Installing shuu ${version}..."

  filename="shuu-${os}-${arch}.tar.gz"
  url="https://github.com/${REPO}/releases/download/${version}/${filename}"

  # Download
  local tmpdir
  tmpdir="$(mktemp -d)"
  trap 'rm -rf "$tmpdir"' EXIT

  if ! curl -fsSL -o "${tmpdir}/${filename}" "$url"; then
    err "Download failed: ${url}\nMake sure release ${version} exists and has ${filename}."
  fi

  # Extract
  tar xzf "${tmpdir}/${filename}" -C "$tmpdir"

  # Install
  mkdir -p "$INSTALL_DIR"
  mv "${tmpdir}/shuu" "${INSTALL_DIR}/shuu"
  chmod 755 "${INSTALL_DIR}/shuu"

  success "Installed shuu to ${INSTALL_DIR}/shuu"

  # Add to PATH if needed
  if echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
    return 0
  fi

  local shell_config
  shell_config="$(detect_shell_config)"
  local path_line="export PATH=\"${INSTALL_DIR}:\$PATH\""
  local shell_name
  shell_name="$(basename "${SHELL:-/bin/sh}")"

  if [ "$shell_name" = "fish" ]; then
    path_line="fish_add_path ${INSTALL_DIR}"
  fi

  if [ -f "$shell_config" ] && grep -qF "$INSTALL_DIR" "$shell_config" 2>/dev/null; then
    return 0
  fi

  printf '\n# shuu\n%s\n' "$path_line" >> "$shell_config"
  info "Added ${INSTALL_DIR} to PATH in ${shell_config}"
  info "Run 'source ${shell_config}' or restart your shell to use shuu."
}

main "$@"
