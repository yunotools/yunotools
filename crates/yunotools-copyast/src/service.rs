use crate::scanner::Scanner;
use crate::writer::Writer;
use std::path::Path;
use yunotools_core::logger;

pub struct CopyastService;

impl CopyastService {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
        logger::info("Copyast scanning started");

        let files = Scanner::scan(Path::new(input));

        logger::info(&format!("Collected {} files", files.len()));

        Writer::write(&files, output)?;

        logger::success("Copyast finished");

        Ok(())
    }
}
