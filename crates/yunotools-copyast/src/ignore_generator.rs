use crate::{CopyastError, IgnoreTemplate};
use std::fs;

pub struct IgnoreGenerator;

impl IgnoreGenerator {
    pub fn generate(template_name: &str) -> Result<(), CopyastError> {
        let content = IgnoreTemplate::generate(template_name).ok_or_else(|| {
            CopyastError::UnknownIgnoreTemplate {
                name: template_name.to_owned(),
            }
        })?;

        fs::write(".yunotools-ignore", content)?;

        Ok(())
    }
}
