use super::catalog;

pub fn generate() -> String {
    let mut result = String::new();

    for (index, template) in catalog::all().iter().enumerate() {
        if index > 0 {
            result.push('\n');
        }

        result.push_str("# =========================\n# ");
        result.push_str(template.title);
        result.push_str("\n# =========================\n");
        result.push_str(template.content.trim());
        result.push('\n');
    }

    result
}
