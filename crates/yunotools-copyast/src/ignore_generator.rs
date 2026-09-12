use crate::IgnoreTemplate;
use std::fs;

pub struct IgnoreGenerator;

impl IgnoreGenerator {
    pub fn generate(kind: &str) -> Result<(), String> {
        let content =
            IgnoreTemplate::generate(kind).ok_or(format!("Unknown template: {}", kind))?;

        fs::write(".yunotools-ignore", content).map_err(|e| e.to_string())?;

        Ok(())
    }
}
