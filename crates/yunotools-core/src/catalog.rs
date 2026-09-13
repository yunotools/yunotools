use crate::ModuleMetadata;

// Danh bạ chứa thông tin của các module đã được đăng ký.
#[derive(Default)]
pub struct ModuleCatalog {
    modules: Vec<ModuleMetadata>,
}

impl ModuleCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add_module(&mut self, metadata: ModuleMetadata) {
        self.modules.push(metadata);
    }

    pub fn list_modules(&self) -> &[ModuleMetadata] {
        &self.modules
    }

    pub fn search_modules(&self, query: &str) -> Vec<ModuleMetadata> {
        let normalized_query = query.trim().to_ascii_lowercase();

        if normalized_query.is_empty() {
            return self.modules.clone();
        }

        self.modules
            .iter()
            .copied()
            .filter(|metadata| {
                metadata
                    .name
                    .to_ascii_lowercase()
                    .contains(&normalized_query)
                    || metadata
                        .description
                        .to_ascii_lowercase()
                        .contains(&normalized_query)
                    || metadata
                        .command_flags
                        .iter()
                        .any(|selector| selector.to_ascii_lowercase().contains(&normalized_query))
            })
            .collect()
    }
}
