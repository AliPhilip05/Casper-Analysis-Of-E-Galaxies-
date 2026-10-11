mod input;
mod output;

pub use input::read_galaxies;
pub use output::{timestamped_output_path, write_latest_file, write_results};
