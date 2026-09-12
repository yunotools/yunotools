use crate::domain::TextFile;
use std::fs::File;
use std::io::Write;

pub struct Writer;

impl Writer {
    pub fn write(files: &Vec<TextFile>, output: &str) -> std::io::Result<()> {
        let mut file = File::create(output)?;

        for item in files {
            writeln!(
                file,
                "\n******************Yunotools-Copyast******************"
            )?;

            writeln!(
                file,
                "******************{}******************",
                item.path.display()
            )?;

            writeln!(file, "{}", item.content)?;
        }

        Ok(())
    }
}
