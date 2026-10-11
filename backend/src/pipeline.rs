use crate::analysis::calculate_results;
use crate::io::{read_galaxies, timestamped_output_path, write_latest_file, write_results};
use std::error::Error;
use std::path::{Path, PathBuf};

/// Summary returned after one input CSV has been processed.
#[derive(Debug)]
pub struct RunSummary {
    pub galaxy_count: usize,
    pub result_count: usize,
    pub output_path: PathBuf,
}

/// Run the bundled sample and write to `backend/data/results/`.
pub fn run_default() -> Result<RunSummary, Box<dyn Error>> {
    let backend_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input_path = backend_dir.join("data/input/selected_galaxies_dp1.csv");
    let output_dir = backend_dir.join("data/results");
    let summary = run_to_directory(&input_path, &output_dir)?;
    write_latest_file(&output_dir, &summary.output_path)?;
    Ok(summary)
}

/// Run an input CSV and create a timestamped output file in `output_dir`.
pub fn run_to_directory(
    input_path: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<RunSummary, Box<dyn Error>> {
    let output_path = timestamped_output_path(output_dir);
    run_with_paths(input_path, output_path)
}

/// Run an input CSV and write results to the exact output path supplied.
pub fn run_with_paths(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<RunSummary, Box<dyn Error>> {
    let galaxies = read_galaxies(input_path)?;
    let results = calculate_results(&galaxies);
    write_results(&results, output_path.as_ref())?;

    Ok(RunSummary {
        galaxy_count: galaxies.len(),
        result_count: results.len(),
        output_path: output_path.as_ref().to_path_buf(),
    })
}
