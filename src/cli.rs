use clap::Parser;
use std::path::PathBuf;
use colored::Colorize;
use crate::error::Result;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(help = "Path to the Rust types file")]
    pub types_file: PathBuf,

    #[arg(short, long, help = "Output directory for generated tests", default_value = "tests")]
    pub output_dir: PathBuf,

    #[arg(short, long, help = "Verbose output")]
    pub verbose: bool,
}

impl Cli {
    pub fn validate(&self) -> Result<()> {
        if !self.types_file.exists() {
            return Err(crate::error::TypeTesterError::FileReadError(
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("File not found: {}", self.types_file.display()),
                ),
            ));
        }

        if !self.types_file.extension().map_or(false, |ext| ext == "rs") {
            return Err(crate::error::TypeTesterError::InvalidFileType(
                format!("File must have .rs extension: {}", self.types_file.display()),
            ));
        }

        Ok(())
    }

    pub fn ensure_output_dir(&self) -> Result<()> {
        if !self.output_dir.exists() {
            std::fs::create_dir_all(&self.output_dir)?;
        }
        Ok(())
    }

    pub fn log_info(&self, message: &str) {
        if self.verbose {
            println!("{} {}", "INFO:".blue().bold(), message);
        }
    }

    pub fn log_success(&self, message: &str) {
        println!("{} {}", "SUCCESS:".green().bold(), message);
    }

    pub fn log_error(&self, message: &str) {
        eprintln!("{} {}", "ERROR:".red().bold(), message);
    }
} 