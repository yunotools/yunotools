use super::catalog;

pub fn generate() -> String {
    render(catalog::all())
}

pub(crate) fn render<'a>(
    templates: impl IntoIterator<Item = &'a catalog::TemplateDefinition>,
) -> String {
    let mut result = String::new();

    for (index, template) in templates.into_iter().enumerate() {
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
