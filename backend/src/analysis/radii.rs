/// Estimate r80 and r20 for each flux using the catalog shape moments.
pub fn estimate_radii(
    flux_array: &[f64],
    shape_xx_array: &[f64],
    shape_yy_array: &[f64],
) -> (Vec<f64>, Vec<f64>) {
    let mut r_80 = Vec::with_capacity(flux_array.len());
    let mut r_20 = Vec::with_capacity(flux_array.len());

    let max_flux = flux_array.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_flux = flux_array.iter().copied().fold(f64::INFINITY, f64::min);
    let flux_range = max_flux - min_flux;

    for (index, flux) in flux_array.iter().copied().enumerate() {
        let total_radius = (shape_xx_array[index] + shape_yy_array[index]).abs().sqrt();
        let normalized_flux = if flux_range != 0.0 {
            (flux - min_flux) / flux_range
        } else {
            0.5
        };

        r_80.push((total_radius * (0.5 + 0.5 * normalized_flux)).max(1e-6));
        r_20.push((total_radius * (0.1 + 0.3 * normalized_flux)).max(1e-6));
    }

    (r_80, r_20)
}
