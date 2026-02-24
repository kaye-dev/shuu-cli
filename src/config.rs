use crate::i18n::Lang;
use std::fs;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("shuu");
    }
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(".config").join("shuu")
}

pub fn get_lang() -> Option<String> {
    fs::read_to_string(config_dir().join("lang")).ok()
}

pub fn set_lang(lang: &str) {
    let dir = config_dir();
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(dir.join("lang"), lang);
}

pub fn get_model() -> Option<String> {
    fs::read_to_string(config_dir().join("model"))
        .ok()
        .filter(|s| !s.is_empty())
}

pub fn set_model(model: &str) {
    let dir = config_dir();
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(dir.join("model"), model);
}

pub fn reset() {
    let dir = config_dir();
    let _ = fs::remove_file(dir.join("lang"));
    let _ = fs::remove_file(dir.join("model"));
}

fn parse_lang(s: &str) -> Option<Lang> {
    let s = s.trim();
    for &(code, lang) in crate::i18n::LANG_CODES {
        if code == s {
            return Some(lang);
        }
    }
    None
}

pub fn resolve_lang() -> Lang {
    // Priority: SHUU_LANG env > config file > LANG env > English
    if let Ok(env_lang) = std::env::var("SHUU_LANG") {
        if let Some(lang) = parse_lang(&env_lang) {
            return lang;
        }
    }

    if let Some(config_lang) = get_lang() {
        if let Some(lang) = parse_lang(&config_lang) {
            return lang;
        }
    }

    if let Ok(env_lang) = std::env::var("LANG") {
        let short = env_lang.split('.').next().unwrap_or("");
        let short = short.split('_').next().unwrap_or("");
        if let Some(lang) = parse_lang(short) {
            return lang;
        }
    }

    Lang::En
}

pub fn check_first_run() {
    if config_dir().join("lang").exists() {
        return;
    }

    // Language selection
    let items: Vec<String> = crate::i18n::LANG_NAMES.iter().map(|s| s.to_string()).collect();
    let selected = match crate::menu::select_menu(
        "Select language / \u{8a00}\u{8a9e}\u{9078}\u{629e}",
        &items,
        "\u{2191}\u{2193}: move  Enter: select  q: cancel",
    ) {
        Some(idx) => idx,
        None => std::process::exit(0),
    };

    let chosen_code = crate::i18n::LANG_CODES[selected].0;
    set_lang(chosen_code);

    // Model selection (now in chosen language)
    let lang = crate::i18n::LANG_CODES[selected].1;
    let messages = crate::i18n::messages(lang);

    let model_ids = [
        "claude-sonnet-4-6",
        "claude-opus-4-6",
        "claude-haiku-4-5-20251001",
    ];
    let model_labels: Vec<String> = vec![
        "Sonnet 4.6  (claude-sonnet-4-6)".to_string(),
        "Opus 4.6    (claude-opus-4-6)".to_string(),
        "Haiku 4.5   (claude-haiku-4-5-20251001)".to_string(),
    ];

    let model_selected = match crate::menu::select_menu(
        messages.model_select,
        &model_labels,
        messages.menu_hint,
    ) {
        Some(idx) => idx,
        None => std::process::exit(0),
    };

    set_model(model_ids[model_selected]);
    crate::success(&messages.model_set.replacen("%s", model_ids[model_selected], 1));
    eprintln!();
}
