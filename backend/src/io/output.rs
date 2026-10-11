use crate::model::CasResult;
use chrono::Local;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// Build a timestamped result path inside the requested directory.
pub fn timestamped_output_path(directory: impl AsRef<Path>) -> PathBuf {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    directory
        .as_ref()
        .join(format!("cas_results_{timestamp}.csv"))
}

/// Write CAS results to the exact path provided by the caller.
pub fn write_results(
    results: &[CasResult],
    output_path: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(output_path)?;
    let mut writer = WriterBuilder::new().from_writer(file);
    writer.write_record(["index", "objectId", "band", "C", "A", "S"])?;

    for result in results {
        writer.write_record([
            result.index.to_string(),
            result.object_id.clone(),
            result.band.clone(),
            result.concentration.to_string(),
            result.asymmetry.to_string(),
            result.smoothness.to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

/// Update the pointer used by the default bundled-data workflow.
pub fn write_latest_file(
    directory: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    let filename = output_path
        .as_ref()
        .file_name()
        .ok_or("Output path does not have a filename")?;
    fs::create_dir_all(directory.as_ref())?;
    fs::write(
        directory.as_ref().join("latest_file.txt"),
        filename.to_string_lossy().as_bytes(),
    )?;
    Ok(())
}
