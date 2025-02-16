mod analyzer;
mod cli;
mod error;
mod generator;

use clap::Parser;
use error::{Result, TypeTesterError};
use generator::TestConfig;
use std::process::Command;

fn main() -> Result<()> {
    match run() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    cli.validate()?;
    cli.ensure_output_dir()?;

    cli.log_info(&format!("Analyzing file: {}", cli.types_file.display()));

    let analyzer = analyzer::TypeAnalyzer::new();
    let types = analyzer
        .analyze_file(&cli.types_file)
        .map_err(|e| TypeTesterError::AnalyzerError(format!("Failed to analyze types: {}", e)))?;

    cli.log_info(&format!("Found {} type definitions", types.len()));

    let config = TestConfig {
        check_derives: true,
        check_serialization: true,
        check_size: true,
        check_fields: true,
    };

    let test_suite = generator::TestSuite::with_config(config);
    let tests = test_suite.generate_tests(&types).map_err(|e| {
        TypeTesterError::GenerationError(format!("Failed to generate tests: {}", e))
    })?;

    let output_path = cli.output_dir.join("generated_tests.rs");
    std::fs::write(&output_path, tests).map_err(|e| TypeTesterError::IoError(e))?;

    cli.log_info("Formatting generated tests...");
    match Command::new("cargo")
        .args(["fmt", "--", &output_path.to_string_lossy()])
        .output()
    {
        Ok(_) => cli.log_info("Tests formatted successfully"),
        Err(e) => cli.log_error(&format!("Failed to format tests: {}", e)),
    }

    cli.log_success(&format!(
        "Generated tests written to: {}",
        output_path.display()
    ));

    Ok(())
}
