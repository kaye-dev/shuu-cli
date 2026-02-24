use crate::i18n;
use crate::menu::select_menu;
use crate::{ai, banner, config, git};
use crate::{err, info, success, App};
use crate::{BOLD, BLUE, CYAN, DIM, GREEN, NC, RED, VERSION, YELLOW};
use std::io::{self, Write};

fn read_line_prompt(prompt: &str) -> String {
    eprint!("{BOLD}{prompt}{NC} ");
    let _ = io::stderr().flush();
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    line.trim().to_string()
}

fn confirm(prompt: &str) -> bool {
    eprint!("{BOLD}{prompt}{NC} ");
    let _ = io::stderr().flush();
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let answer = line.trim();
    answer == "y" || answer == "Y"
}

fn require_git_repo(app: &App) {
    if !git::is_git_repo() {
        err(app.messages.not_git_repo);
        std::process::exit(1);
    }
}

pub fn cmd_create(app: &App, description: String) {
    require_git_repo(app);
    let m = app.messages;

    let worktrees_dir = match git::worktrees_dir() {
        Some(d) => d,
        None => {
            err("Could not determine worktrees directory");
            std::process::exit(1);
        }
    };

    let description = if description.is_empty() {
        let desc = read_line_prompt(m.what_to_implement);
        if desc.is_empty() {
            err(m.enter_desc);
            std::process::exit(1);
        }
        desc
    } else {
        description
    };

    let mut suggested_name = ai::generate_branch_name(&description, "", m);

    let branch_name = loop {
        let items = vec![
            suggested_name.clone(),
            m.enter_manually.to_string(),
            m.regenerate.to_string(),
        ];

        let choice = match select_menu(m.select_branch, &items, m.menu_hint) {
            Some(c) => c,
            None => std::process::exit(1),
        };

        match choice {
            0 => break suggested_name.clone(),
            1 => {
                let name = read_line_prompt(m.branch_name);
                if !name.is_empty() {
                    break name;
                }
                err(m.enter_branch_name);
            }
            2 => {
                let fb = read_line_prompt(m.feedback);
                if !fb.is_empty() {
                    suggested_name = ai::generate_branch_name(&description, &fb, m);
                } else {
                    err(m.enter_feedback);
                }
            }
            _ => {}
        }
    };

    // Derive worktree directory name from branch (replace / with -)
    let wt_dirname = branch_name.replace('/', "-");
    let wt_path = format!("{}/{}", worktrees_dir, wt_dirname);

    if std::path::Path::new(&wt_path).is_dir() {
        err(&format!("{}: {}", m.wt_already_exists, wt_path));
        std::process::exit(1);
    }

    let _ = std::fs::create_dir_all(&worktrees_dir);

    info(m.creating_wt);
    match git::worktree_add(&wt_path, &branch_name) {
        Ok(true) => success(m.wt_created),
        Ok(false) => success(m.wt_created_existing),
        Err(()) => {
            err(m.wt_create_failed);
            std::process::exit(1);
        }
    }

    eprintln!();
    eprintln!("  {DIM}{}{NC} {}", m.path, wt_path);
    eprintln!("  {DIM}{}{NC} {}", m.branch, branch_name);
    eprintln!();
    eprintln!("{CYAN}\u{25b8}{NC} {} {BOLD}shuu switch{NC}", m.to_switch);
}

pub fn cmd_list(app: &App) {
    require_git_repo(app);
    let m = app.messages;

    let main_wt = git::main_worktree().unwrap_or_default();
    let worktrees = git::worktree_list();

    eprintln!("\n{BOLD}{}{NC}\n", m.list_title);

    for wt in &worktrees {
        if wt.path == main_wt {
            eprintln!(
                "  {YELLOW}\u{2605}{NC} {BOLD}{:<50}{NC} {DIM}{}{NC}  {GREEN}{}{NC}",
                wt.path, wt.hash, wt.branch
            );
        } else {
            eprintln!(
                "    {:<50} {DIM}{}{NC}  {BLUE}{}{NC}",
                wt.path, wt.hash, wt.branch
            );
        }
    }

    eprintln!();
}

pub fn cmd_remove(app: &App) {
    require_git_repo(app);
    let m = app.messages;

    let main_wt = git::main_worktree().unwrap_or_default();
    let worktrees = git::worktree_list();

    let removable: Vec<_> = worktrees.iter().filter(|wt| wt.path != main_wt).collect();

    if removable.is_empty() {
        info(m.no_wt_to_remove);
        return;
    }

    let labels: Vec<String> = removable
        .iter()
        .map(|wt| format!("{}  {}", wt.branch, wt.path))
        .collect();

    let selected = match select_menu(m.select_wt_remove, &labels, m.menu_hint) {
        Some(s) => s,
        None => return,
    };

    let target = &removable[selected];

    // Confirm
    eprintln!("\n{RED}{BOLD}{}{NC}", m.confirm_delete);
    eprintln!("  {} {}", m.path, target.path);
    eprintln!("  {} {}", m.branch, target.branch);

    if !confirm("[y/N]:") {
        info(m.cancelled);
        return;
    }

    // Remove worktree
    if git::worktree_remove(&target.path) {
        success(&format!("{}: {}", m.wt_removed, target.path));
    } else {
        err(m.wt_remove_failed);
        if confirm(&format!("{} [y/N]:", m.force_remove)) {
            if git::worktree_force_remove(&target.path) {
                success(m.wt_force_removed);
            } else {
                err(m.force_remove_failed);
                std::process::exit(1);
            }
        } else {
            std::process::exit(1);
        }
    }

    // Ask about branch deletion
    let delete_msg = m.delete_branch.replacen("%s", &target.branch, 1);
    if confirm(&format!("{} [y/N]:", delete_msg)) {
        if git::branch_delete(&target.branch) {
            success(&format!("{}: {}", m.branch_deleted, target.branch));
        } else if confirm(&format!("{} [y/N]:", m.force_delete_branch)) {
            git::branch_force_delete(&target.branch);
            success(&format!("{}: {}", m.branch_force_deleted, target.branch));
        }
    }
}

pub fn cmd_switch(app: &App) {
    require_git_repo(app);
    let m = app.messages;

    let worktrees = git::worktree_list();

    if worktrees.len() <= 1 {
        info(m.no_wt_to_switch);
        return;
    }

    let current_dir = std::env::current_dir()
        .ok()
        .and_then(|p| p.canonicalize().ok())
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let labels: Vec<String> = worktrees
        .iter()
        .map(|wt| {
            let mut label = format!("{}  {}", wt.branch, wt.path);
            if current_dir.starts_with(&wt.path) {
                label.push_str(" (current)");
            }
            label
        })
        .collect();

    let selected = match select_menu(m.select_wt_switch, &labels, m.menu_hint) {
        Some(s) => s,
        None => return,
    };

    let target = &worktrees[selected];
    success(&m.switching_to.replacen("%s", &target.branch, 1));
    let _ = std::fs::write("/tmp/.shuu_cd_target", &target.path);
}

pub fn cmd_settings(app: &App) {
    let m = app.messages;

    let current_lang_code = config::get_lang().unwrap_or_default();
    let mut lang_display = m.model_none.to_string();
    for (i, &(code, _)) in i18n::LANG_CODES.iter().enumerate() {
        if code == current_lang_code {
            lang_display = i18n::LANG_NAMES[i].to_string();
            break;
        }
    }

    let model_display = config::get_model().unwrap_or_else(|| m.model_none.to_string());

    let items = vec![
        format!("{}  ({})", m.settings_lang, lang_display),
        format!("{}  ({})", m.settings_model, model_display),
        m.settings_reset.to_string(),
    ];

    let selected = match select_menu(m.settings_title, &items, m.menu_hint) {
        Some(s) => s,
        None => return,
    };

    match selected {
        0 => select_lang(app),
        1 => select_model(app),
        2 => {
            config::reset();
            success(m.settings_reset_done);
        }
        _ => {}
    }
}

fn select_lang(app: &App) {
    let m = app.messages;
    let current = config::get_lang().unwrap_or_default();

    let items: Vec<String> = i18n::LANG_CODES
        .iter()
        .enumerate()
        .map(|(i, &(code, _))| {
            let mut label = i18n::LANG_NAMES[i].to_string();
            if code == current {
                label.push_str(" \u{2605}");
            }
            label
        })
        .collect();

    let selected = match select_menu(m.settings_lang, &items, m.menu_hint) {
        Some(s) => s,
        None => return,
    };

    let (new_code, new_lang) = i18n::LANG_CODES[selected];
    config::set_lang(new_code);
    let new_messages = i18n::messages(new_lang);
    success(
        &new_messages
            .lang_set
            .replacen("%s", i18n::LANG_NAMES[selected], 1),
    );
}

fn select_model(app: &App) {
    let m = app.messages;
    let current = config::get_model().unwrap_or_default();

    let model_ids = [
        "claude-sonnet-4-6",
        "claude-opus-4-6",
        "claude-haiku-4-5-20251001",
    ];
    let model_labels = [
        "Sonnet 4.6  (claude-sonnet-4-6)",
        "Opus 4.6    (claude-opus-4-6)",
        "Haiku 4.5   (claude-haiku-4-5-20251001)",
    ];

    let items: Vec<String> = model_ids
        .iter()
        .enumerate()
        .map(|(i, &id)| {
            let mut label = model_labels[i].to_string();
            if id == current {
                label.push_str(" \u{2605}");
            }
            label
        })
        .collect();

    let selected = match select_menu(m.model_select, &items, m.menu_hint) {
        Some(s) => s,
        None => return,
    };

    config::set_model(model_ids[selected]);
    success(&m.model_set.replacen("%s", model_ids[selected], 1));
}

pub fn cmd_help(app: &App) {
    let m = app.messages;
    eprintln!();
    eprintln!("{BOLD}shuu{NC} - Git Worktree Manager v{VERSION}");
    eprintln!();
    eprintln!("{BOLD}{}:{NC}", m.help_usage);
    eprintln!("    shuu <command>");
    eprintln!(
        "    shuu \"<description>\"       {}",
        m.help_direct_desc
    );
    eprintln!();
    eprintln!("{BOLD}{}:{NC}", m.help_commands);
    eprintln!(
        "    {GREEN}create{NC}  (c)      {}",
        m.help_create_desc
    );
    eprintln!(
        "    {GREEN}list{NC}    (l, ls)  {}",
        m.help_list_desc
    );
    eprintln!(
        "    {GREEN}remove{NC}  (rm)     {}",
        m.help_remove_desc
    );
    eprintln!(
        "    {GREEN}switch{NC}  (s)      {}",
        m.help_switch_desc
    );
    eprintln!(
        "    {GREEN}settings{NC}         {}",
        m.help_settings_desc
    );
    eprintln!(
        "    {GREEN}help{NC}    (-h)     {}",
        m.help_help_desc
    );
    eprintln!();
    eprintln!("{BOLD}{}:{NC}", m.help_examples);
    eprintln!("    shuu                      # {}", m.menu_title);
    eprintln!("    shuu create               # {}", m.help_create_desc);
    eprintln!(
        "    shuu \"implement auth\"     # {}",
        m.help_direct_desc
    );
    eprintln!("    shuu ls                   # {}", m.help_list_desc);
    eprintln!("    shuu s                    # {}", m.help_switch_desc);
    eprintln!("    shuu rm                   # {}", m.help_remove_desc);
    eprintln!();
    eprintln!("{BOLD}{}:{NC}", m.help_notes);
    eprintln!("    {}", m.help_note_path);
    eprintln!("    {}", m.help_note_switch);
    eprintln!();
}

pub fn cmd_interactive(app: &App) {
    banner::show_banner(app);
    let m = app.messages;

    let items = vec![
        m.menu_create.to_string(),
        m.menu_list.to_string(),
        m.menu_remove.to_string(),
        m.menu_switch.to_string(),
        m.menu_settings.to_string(),
        m.menu_help.to_string(),
    ];

    let selected = match select_menu(m.menu_title, &items, m.menu_hint) {
        Some(s) => s,
        None => return,
    };

    match selected {
        0 => cmd_create(app, String::new()),
        1 => cmd_list(app),
        2 => cmd_remove(app),
        3 => cmd_switch(app),
        4 => cmd_settings(app),
        5 => cmd_help(app),
        _ => {}
    }
}
