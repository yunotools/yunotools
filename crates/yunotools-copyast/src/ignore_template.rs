use crate::templates;

pub struct IgnoreTemplate;

impl IgnoreTemplate {
    pub fn generate(template_name: &str) -> Option<String> {
        if template_name.trim().eq_ignore_ascii_case("all") {
            return Some(templates::all::generate());
        }

        templates::catalog::find(template_name)
            .map(|template| template.content.trim().to_owned() + "\n")
    }

    pub fn available() -> Vec<&'static str> {
        templates::catalog::all()
            .iter()
            .map(|template| template.name)
            .chain(std::iter::once("all"))
            .collect()
    }
}
