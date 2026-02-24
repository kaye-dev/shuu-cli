use std::process::{Command, Stdio};

fn claude_available() -> bool {
    std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).any(|dir| dir.join("claude").exists()))
        .unwrap_or(false)
}

fn build_prompt(description: &str, feedback: &str) -> String {
    let mut prompt = format!(
        "Suggest exactly one git branch name for the following implementation.\n\
         Rules:\n\
         - kebab-case\n\
         - English\n\
         - Short and concise (2-4 words)\n\
         - Use prefix: feat/, fix/, refactor/, etc.\n\
         - Output only the branch name (no explanation)\n\
         \n\
         Implementation: {}",
        description
    );

    if !feedback.is_empty() {
        prompt.push_str(&format!(
            "\n\nFeedback on previous suggestion: {}",
            feedback
        ));
    }

    prompt
}

fn fallback_branch_name(description: &str) -> String {
    let kebab: String = description
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let kebab = kebab.trim_matches('-').replace("--", "-");
    let kebab = if kebab.len() > 40 {
        &kebab[..40]
    } else {
        &kebab
    };
    format!("feat/{}", kebab.trim_end_matches('-'))
}

pub fn generate_branch_name(
    description: &str,
    feedback: &str,
    messages: &crate::i18n::Messages,
) -> String {
    if claude_available() {
        crate::info(messages.generating_branch);

        let prompt = build_prompt(description, feedback);
        let mut cmd = Command::new("claude");
        if let Some(model) = crate::config::get_model() {
            cmd.args(["--model", &model]);
        }
        cmd.args(["-p", &prompt]);
        cmd.env("CLAUDECODE", "");
        cmd.stderr(Stdio::null());

        if let Ok(output) = cmd.output() {
            if output.status.success() {
                let name = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if !name.is_empty() {
                    return name;
                }
            }
        }
    }

    fallback_branch_name(description)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_prompt_without_feedback() {
        let prompt = build_prompt("add login feature", "");
        assert!(prompt.contains("add login feature"));
        assert!(!prompt.contains("Feedback"));
    }

    #[test]
    fn test_build_prompt_with_feedback() {
        let prompt = build_prompt("add login feature", "use oauth instead");
        assert!(prompt.contains("add login feature"));
        assert!(prompt.contains("Feedback on previous suggestion: use oauth instead"));
    }

    #[test]
    fn test_build_prompt_contains_rules() {
        let prompt = build_prompt("anything", "");
        assert!(prompt.contains("kebab-case"));
        assert!(prompt.contains("feat/, fix/, refactor/"));
    }

    #[test]
    fn test_fallback_branch_name_simple() {
        assert_eq!(fallback_branch_name("add login"), "feat/add-login");
    }

    #[test]
    fn test_fallback_branch_name_uppercase() {
        assert_eq!(
            fallback_branch_name("Add Login Feature"),
            "feat/add-login-feature"
        );
    }

    #[test]
    fn test_fallback_branch_name_special_chars() {
        // `:` and `'` and `!` become `-`, then `--` is collapsed to `-`
        assert_eq!(
            fallback_branch_name("fix: user's email bug!"),
            "feat/fix-user-s-email-bug"
        );
    }

    #[test]
    fn test_fallback_branch_name_long_description() {
        let long_desc =
            "this is a very long description that should be truncated at forty characters exactly";
        let result = fallback_branch_name(long_desc);
        // "feat/" prefix + at most 40 chars
        assert!(result.len() <= 45);
        assert!(result.starts_with("feat/"));
    }

    #[test]
    fn test_fallback_branch_name_leading_trailing_special() {
        // Leading/trailing spaces become `-`, then trim_matches('-') removes them
        assert_eq!(fallback_branch_name("  hello world  "), "feat/hello-world");
    }
}
