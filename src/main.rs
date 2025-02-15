mod analyzer;
mod cli;
mod error;
mod generator;

use clap::Parser;
use error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    cli.validate()?;
    cli.ensure_output_dir()?;

    cli.log_info(&format!("Analyzing file: {}", cli.types_file.display()));
    
    let analyzer = analyzer::TypeAnalyzer::new();
    let types = analyzer.analyze_file(&cli.types_file)?;

    cli.log_info(&format!("Found {} type definitions", types.len()));

    let test_suite = generator::TestSuite::new();
    let tests = test_suite.generate_tests(&types)?;

    let output_path = cli.output_dir.join("generated_tests.rs");
    std::fs::write(&output_path, tests)?;

    cli.log_success(&format!(
        "Generated tests written to: {}",
        output_path.display()
    ));

    Ok(())
}
