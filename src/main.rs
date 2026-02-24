mod ai;
mod banner;
mod commands;
mod config;
mod git;
mod i18n;
mod menu;

pub const RED: &str = "\x1b[0;31m";
pub const GREEN: &str = "\x1b[0;32m";
pub const YELLOW: &str = "\x1b[0;33m";
pub const BLUE: &str = "\x1b[0;34m";
pub const CYAN: &str = "\x1b[0;36m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const NC: &str = "\x1b[0m";

pub const VERSION: &str = "2.1.0";

pub struct App {
    #[allow(dead_code)]
    pub lang: i18n::Lang,
    pub messages: &'static i18n::Messages,
}

pub fn err(msg: &str) {
    eprintln!("{}error:{} {}", RED, NC, msg);
}

pub fn info(msg: &str) {
    eprintln!("{}\u{25b8}{} {}", CYAN, NC, msg);
}

pub fn success(msg: &str) {
    eprintln!("{}\u{2713}{} {}", GREEN, NC, msg);
}

fn main() {
    config::check_first_run();

    let lang = config::resolve_lang();
    let messages = i18n::messages(lang);
    let app = App { lang, messages };

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        None => commands::cmd_interactive(&app),
        Some("create" | "c") => {
            let desc = if args.len() > 2 {
                args[2..].join(" ")
            } else {
                String::new()
            };
            commands::cmd_create(&app, desc);
        }
        Some("list" | "l" | "ls") => commands::cmd_list(&app),
        Some("remove" | "rm") => commands::cmd_remove(&app),
        Some("switch" | "s") => commands::cmd_switch(&app),
        Some("settings") => commands::cmd_settings(&app),
        Some("help" | "-h" | "--help") => commands::cmd_help(&app),
        Some(_) => {
            let desc = args[1..].join(" ");
            commands::cmd_create(&app, desc);
        }
    }
}
