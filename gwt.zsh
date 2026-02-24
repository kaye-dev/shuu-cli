# gwt - Git Worktree Manager shell wrapper
# Source this file in your .zshrc:
#   source ~/shuu-cli/gwt.zsh

gwt() {
    rm -f /tmp/.gwt_cd_target
    command gwt "$@"
    if [[ -f /tmp/.gwt_cd_target ]]; then
        local target
        target=$(cat /tmp/.gwt_cd_target)
        rm -f /tmp/.gwt_cd_target
        [[ -n "$target" && -d "$target" ]] && cd "$target"
    fi
}
