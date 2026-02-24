# shuu - Git Worktree Manager shell wrapper
# Source this file in your .zshrc:
#   source ~/shuu-cli/shuu.zsh

shuu() {
    rm -f /tmp/.shuu_cd_target
    command shuu "$@"
    if [[ -f /tmp/.shuu_cd_target ]]; then
        local target
        target=$(cat /tmp/.shuu_cd_target)
        rm -f /tmp/.shuu_cd_target
        [[ -n "$target" && -d "$target" ]] && cd "$target"
    fi
}
