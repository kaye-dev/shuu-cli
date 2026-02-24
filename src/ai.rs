use std::process::{Command, Stdio};

fn claude_available() -> bool {
    std::env::var_os("PATH")
        .map(|path| {
            std::env::split_paths(&path).any(|dir| dir.join("claude").exists())
        })
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
