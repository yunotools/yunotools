use super::template::IgnoreTemplate;
use crate::CopyastError;
use std::fs;
use std::path::Path;

pub struct IgnoreGenerator;

impl IgnoreGenerator {
    pub fn generate(template_name: &str) -> Result<(), CopyastError> {
        Self::generate_to(template_name, ".yunotools-ignore")
    }

    pub fn generate_to(template_name: &str, output: impl AsRef<Path>) -> Result<(), CopyastError> {
        let content = IgnoreTemplate::generate(template_name).ok_or_else(|| {
            CopyastError::UnknownIgnoreTemplate {
                name: template_name.to_owned(),
            }
        })?;

        let output = output.as_ref();

        if let Some(parent) = output.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)?;
        }

        fs::write(output, content)?;

        Ok(())
    }
}
