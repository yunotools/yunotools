use crate::templates;
use std::collections::HashSet;

pub struct IgnoreTemplate;

impl IgnoreTemplate {
    /// Nhận một template (`rust`) hoặc nhiều template
    /// ngăn cách bằng dấu phẩy/dấu cộng (`rust,tauri`, `python+django`).
    pub fn generate(template_selection: &str) -> Option<String> {
        let requested_names = template_selection
            .split([',', '+'])
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .collect::<Vec<_>>();

        if requested_names.is_empty() {
            return None;
        }

        if requested_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case("all"))
        {
            return Some(templates::all::generate());
        }

        let mut seen_names = HashSet::new();
        let mut selected_templates = Vec::new();

        for requested_name in requested_names {
            let template = templates::catalog::find(requested_name)?;

            if seen_names.insert(template.name) {
                selected_templates.push(template);
            }
        }

        if selected_templates.len() == 1 {
            return selected_templates
                .first()
                .map(|template| template.content.trim().to_owned() + "\n");
        }

        Some(templates::all::render(selected_templates))
    }

    pub fn available() -> Vec<&'static str> {
        templates::catalog::all()
            .iter()
            .map(|template| template.name)
            .chain(std::iter::once("all"))
            .collect()
    }
}
