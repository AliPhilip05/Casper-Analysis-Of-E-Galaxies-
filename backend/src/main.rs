use std::error::Error;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();

    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        print_usage();
        return Ok(());
    }

    let summary = match args.as_slice() {
        [] => cas_analysis::run_default()?,
        [input] => {
            let input_path = PathBuf::from(input.as_os_str());
            let output_dir = input_path
                .parent()
                .filter(|path| !path.as_os_str().is_empty())
                .unwrap_or_else(|| Path::new("."));
            cas_analysis::run_to_directory(&input_path, output_dir)?
        }
        [input, output] => cas_analysis::run_with_paths(
            PathBuf::from(input.as_os_str()),
            PathBuf::from(output.as_os_str()),
        )?,
        _ => {
            print_usage();
            return Err("expected an input CSV and an optional output CSV".into());
        }
    };

    println!("Loaded {} galaxies", summary.galaxy_count);
    println!(
        "Wrote {} measurements to {}",
        summary.result_count,
        summary.output_path.display()
    );
    Ok(())
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  cas_analysis                    Process the bundled sample CSV");
    eprintln!("  cas_analysis <input.csv>        Process a CSV and save beside it");
    eprintln!("  cas_analysis <input.csv> <output.csv>");
}
