use crate::{config, git, App, BOLD, DIM, GREEN, NC, VERSION, YELLOW};

pub fn show_banner(app: &App) {
    let model = config::get_model().unwrap_or_else(|| app.messages.model_none.to_string());

    let (repo_info, branch_info, wt_count) = if git::is_git_repo() {
        let repo = git::repo_name().unwrap_or_else(|| "-".to_string());
        let branch = git::current_branch().unwrap_or_else(|| "-".to_string());
        let count = git::worktree_list().len().to_string();
        (repo, branch, count)
    } else {
        ("-".to_string(), "-".to_string(), "-".to_string())
    };

    eprintln!();
    eprintln!(
        "  {DIM}--{NC} {YELLOW}{BOLD}shuu{NC} {DIM}v{VERSION} ------------------------------------{NC}"
    );
    eprintln!();
    eprintln!("  {GREEN}      /\\{NC}           {BOLD}Git Worktree Manager{NC}");
    eprintln!("  {GREEN}     /  \\{NC}");
    eprintln!("  {GREEN}    /    \\{NC}         {DIM}Model  {NC} {model}");
    eprintln!(
        "  {GREEN}   /______\\{NC}        {DIM}Repo   {NC} {repo_info} {DIM}({branch_info}){NC}"
    );
    eprintln!("  {GREEN}      ||{NC}           {DIM}Trees  {NC} {wt_count}");
    eprintln!();
    eprintln!("  {DIM}----------------------------------------------{NC}");
}
