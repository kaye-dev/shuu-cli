use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn main() {
    let locale_dir = Path::new("locale");
    println!("cargo:rerun-if-changed=locale/");

    let mut langs: BTreeMap<String, (String, BTreeMap<String, String>)> = BTreeMap::new();

    for entry in fs::read_dir(locale_dir).expect("Failed to read locale/") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("yml") {
            continue;
        }
        let code = path.file_stem().unwrap().to_str().unwrap().to_string();

        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
        let yaml: serde_yaml::Value = serde_yaml::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e));

        let name = yaml["name"]
            .as_str()
            .unwrap_or_else(|| panic!("{}: missing 'name' field", path.display()))
            .to_string();

        let messages_map = yaml["messages"]
            .as_mapping()
            .unwrap_or_else(|| panic!("{}: missing 'messages' mapping", path.display()));

        let mut messages = BTreeMap::new();
        for (k, v) in messages_map {
            let key = k
                .as_str()
                .unwrap_or_else(|| panic!("{}: non-string key in messages", path.display()))
                .to_string();
            let val = v
                .as_str()
                .unwrap_or_else(|| panic!("{}: non-string value for key '{}'", path.display(), key))
                .to_string();
            messages.insert(key, val);
        }

        langs.insert(code, (name, messages));
    }

    // Validate: all languages must have the same keys as en
    let en_keys: Vec<&String> = langs
        .get("en")
        .expect("locale/en.yml is required")
        .1
        .keys()
        .collect();

    for (code, (_, messages)) in &langs {
        if code == "en" {
            continue;
        }
        let lang_keys: Vec<&String> = messages.keys().collect();
        for key in &en_keys {
            if !messages.contains_key(*key) {
                panic!("locale/{}.yml: missing key '{}'", code, key);
            }
        }
        for key in &lang_keys {
            if !langs["en"].1.contains_key(*key) {
                panic!("locale/{}.yml: extra key '{}' not in en.yml", code, key);
            }
        }
    }

    // Generate Rust code
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("i18n_generated.rs");

    let mut code = String::new();

    // LANG_CODES
    code.push_str("pub const LANG_CODES: &[(&str, Lang)] = &[\n");
    for lang_code in langs.keys() {
        let variant = lang_variant(lang_code);
        code.push_str(&format!("    (\"{}\", Lang::{}),\n", lang_code, variant));
    }
    code.push_str("];\n\n");

    // LANG_NAMES
    code.push_str("pub const LANG_NAMES: &[&str] = &[\n");
    for (name, _) in langs.values() {
        code.push_str(&format!("    \"{}\",\n", escape_str(name)));
    }
    code.push_str("];\n\n");

    // Static instances for each language
    for (lang_code, (_, messages)) in &langs {
        let variant = lang_variant(lang_code);
        code.push_str(&format!(
            "static {}: Messages = Messages {{\n",
            variant.to_uppercase()
        ));
        for key in &en_keys {
            let val = &messages[*key];
            code.push_str(&format!("    {}: \"{}\",\n", key, escape_str(val)));
        }
        code.push_str("};\n\n");
    }

    // messages() function
    code.push_str("pub fn messages(lang: Lang) -> &'static Messages {\n");
    code.push_str("    match lang {\n");
    for lang_code in langs.keys() {
        let variant = lang_variant(lang_code);
        code.push_str(&format!(
            "        Lang::{} => &{},\n",
            variant,
            variant.to_uppercase()
        ));
    }
    code.push_str("    }\n");
    code.push_str("}\n");

    fs::write(&out_path, code)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", out_path.display(), e));
}

fn lang_variant(code: &str) -> String {
    let mut chars = code.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

fn escape_str(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
