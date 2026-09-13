use crate::{AppContext, AppError, ModuleCatalog, ModuleMetadata, ToolModule, logger};

#[derive(Default)]
pub struct ModuleRegistry {
    modules: Vec<Box<dyn ToolModule>>,
    catalog: ModuleCatalog,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_module(&mut self, module: impl ToolModule + 'static) {
        self.catalog.add_module(module.metadata());
        self.modules.push(Box::new(module));
    }

    pub fn list_modules(&self) -> &[ModuleMetadata] {
        self.catalog.list_modules()
    }

    pub fn search_modules(&self, query: &str) -> Vec<ModuleMetadata> {
        self.catalog.search_modules(query)
    }

    pub fn has_matching_module(&self, raw_args: &[String]) -> bool {
        self.find_matching_module(raw_args).is_some()
    }

    pub fn execute(&self, context: &AppContext, raw_args: &[String]) -> Result<(), AppError> {
        let module =
            self.find_matching_module(raw_args)
                .ok_or_else(|| AppError::NoMatchingCommand {
                    app_name: context.app_name.clone(),
                })?;

        let metadata = module.metadata();
        logger::debug(&format!(
            "Running module `{}`: {}",
            metadata.name, metadata.description,
        ));

        module.execute(context, raw_args)
    }

    fn find_matching_module(&self, raw_args: &[String]) -> Option<&dyn ToolModule> {
        self.modules
            .iter()
            .find(|module| module.can_handle(raw_args))
            .map(Box::as_ref)
    }
}
