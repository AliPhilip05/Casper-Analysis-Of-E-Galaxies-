mod concentration;
mod radii;
mod smoothing;

use crate::model::{CasResult, Galaxy};

pub use concentration::calculate_c;
pub use radii::estimate_radii;
pub use smoothing::uniform_filter;

/// Calculate CAS values for every galaxy in all six Rubin photometric bands.
pub fn calculate_results(galaxies: &[Galaxy]) -> Vec<CasResult> {
    let shape_xx: Vec<f64> = galaxies.iter().map(|galaxy| galaxy.shape_xx).collect();
    let shape_yy: Vec<f64> = galaxies.iter().map(|galaxy| galaxy.shape_yy).collect();

    let bands: [(&str, fn(&Galaxy) -> f64); 6] = [
        ("u", |galaxy| galaxy.u_flux),
        ("g", |galaxy| galaxy.g_flux),
        ("r", |galaxy| galaxy.r_flux),
        ("i", |galaxy| galaxy.i_flux),
        ("z", |galaxy| galaxy.z_flux),
        ("y", |galaxy| galaxy.y_flux),
    ];

    let mut results = Vec::with_capacity(galaxies.len() * bands.len());

    for (band, get_flux) in bands {
        let flux: Vec<f64> = galaxies.iter().map(get_flux).collect();
        let flux_rotated: Vec<f64> = flux.iter().rev().copied().collect();
        let (r_80, r_20) = estimate_radii(&flux, &shape_xx, &shape_yy);

        for (index, galaxy) in galaxies.iter().enumerate() {
            let flux_value = flux[index];
            let concentration = calculate_c(r_80[index], r_20[index]);

            let asymmetry_denominator = flux_value.abs() + flux_rotated[index].abs();
            let asymmetry = if asymmetry_denominator != 0.0 {
                (flux_value - flux_rotated[index]).abs() / asymmetry_denominator
            } else {
                0.0
            };

            let mut filter_size = (0.3 * r_80[index]).round() as usize;
            filter_size = filter_size.max(3);
            if filter_size % 2 == 0 {
                filter_size += 1;
            }

            let smoothed_flux = uniform_filter(&flux, filter_size);
            let smoothness = if flux_value != 0.0 {
                (flux_value - smoothed_flux[index]).abs() / flux_value.abs()
            } else {
                0.0
            };

            results.push(CasResult {
                index: index + 1,
                object_id: galaxy.object_id.clone(),
                band: band.to_string(),
                concentration: round_to_four_decimals(concentration),
                asymmetry: round_to_four_decimals(asymmetry),
                smoothness: round_to_four_decimals(smoothness),
            });
        }
    }

    results
}

fn round_to_four_decimals(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}
