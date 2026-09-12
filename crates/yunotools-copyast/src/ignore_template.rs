use crate::templates;

pub struct IgnoreTemplate;

impl IgnoreTemplate {
    pub fn generate(name: &str) -> Option<String> {
        match name.to_lowercase().as_str() {
            "default" => Some(templates::default::TEMPLATE.to_string()),
            "rust" => Some(templates::rust::TEMPLATE.to_string()),
            "node" => Some(templates::node::TEMPLATE.to_string()),
            "python" => Some(templates::python::TEMPLATE.to_string()),
            "go" => Some(templates::go::TEMPLATE.to_string()),
            "docker" => Some(templates::docker::TEMPLATE.to_string()),
            "all" => Some(templates::all::generate()),
            _ => None,
        }
    }
}
