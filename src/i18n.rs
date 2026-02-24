#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Ar,
    En,
    Es,
    Fr,
    Ja,
    Ru,
    Zh,
}

pub struct Messages {
    pub not_git_repo: &'static str,
    pub generating_branch: &'static str,
    pub what_to_implement: &'static str,
    pub enter_desc: &'static str,
    pub select_branch: &'static str,
    pub enter_manually: &'static str,
    pub regenerate: &'static str,
    pub branch_name: &'static str,
    pub feedback: &'static str,
    pub enter_branch_name: &'static str,
    pub enter_feedback: &'static str,
    pub wt_already_exists: &'static str,
    pub creating_wt: &'static str,
    pub wt_created: &'static str,
    pub wt_created_existing: &'static str,
    pub wt_create_failed: &'static str,
    pub path: &'static str,
    pub branch: &'static str,
    pub to_switch: &'static str,
    pub list_title: &'static str,
    pub no_wt_to_remove: &'static str,
    pub select_wt_remove: &'static str,
    pub confirm_delete: &'static str,
    pub cancelled: &'static str,
    pub wt_removed: &'static str,
    pub wt_remove_failed: &'static str,
    pub force_remove: &'static str,
    pub wt_force_removed: &'static str,
    pub force_remove_failed: &'static str,
    pub delete_branch: &'static str,
    pub branch_deleted: &'static str,
    pub force_delete_branch: &'static str,
    pub branch_force_deleted: &'static str,
    pub no_wt_to_switch: &'static str,
    pub select_wt_switch: &'static str,
    pub switching_to: &'static str,
    pub menu_title: &'static str,
    pub menu_create: &'static str,
    pub menu_list: &'static str,
    pub menu_remove: &'static str,
    pub menu_switch: &'static str,
    pub menu_help: &'static str,
    pub menu_hint: &'static str,
    pub help_usage: &'static str,
    pub help_commands: &'static str,
    pub help_examples: &'static str,
    pub help_notes: &'static str,
    pub help_create_desc: &'static str,
    pub help_list_desc: &'static str,
    pub help_remove_desc: &'static str,
    pub help_switch_desc: &'static str,
    pub help_help_desc: &'static str,
    pub help_note_path: &'static str,
    pub help_note_switch: &'static str,
    pub help_direct_desc: &'static str,
    pub model_current: &'static str,
    pub model_select: &'static str,
    pub model_set: &'static str,
    pub model_none: &'static str,
    pub settings_title: &'static str,
    pub settings_lang: &'static str,
    pub settings_model: &'static str,
    pub menu_settings: &'static str,
    pub help_settings_desc: &'static str,
    pub lang_current: &'static str,
    pub lang_set: &'static str,
    pub settings_reset: &'static str,
    pub settings_reset_done: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/i18n_generated.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_codes_and_names_same_length() {
        assert_eq!(LANG_CODES.len(), LANG_NAMES.len());
    }

    #[test]
    fn test_lang_codes_contains_en() {
        assert!(LANG_CODES
            .iter()
            .any(|&(code, lang)| code == "en" && lang == Lang::En));
    }

    #[test]
    fn test_lang_codes_contains_ja() {
        assert!(LANG_CODES
            .iter()
            .any(|&(code, lang)| code == "ja" && lang == Lang::Ja));
    }

    #[test]
    fn test_messages_en_fields_not_empty() {
        let m = messages(Lang::En);
        assert!(!m.not_git_repo.is_empty());
        assert!(!m.menu_title.is_empty());
        assert!(!m.menu_hint.is_empty());
        assert!(!m.help_usage.is_empty());
        assert!(!m.settings_title.is_empty());
    }

    #[test]
    fn test_messages_returns_for_all_langs() {
        for &(_, lang) in LANG_CODES {
            let m = messages(lang);
            assert!(!m.not_git_repo.is_empty());
            assert!(!m.menu_title.is_empty());
        }
    }

    #[test]
    fn test_messages_ja_is_japanese() {
        let m = messages(Lang::Ja);
        // Japanese messages should contain Japanese characters
        assert!(m.menu_title.contains("shuu"));
    }

    #[test]
    fn test_messages_different_languages_differ() {
        let en = messages(Lang::En);
        let ja = messages(Lang::Ja);
        // At least some messages should differ
        assert_ne!(en.not_git_repo, ja.not_git_repo);
    }
}
