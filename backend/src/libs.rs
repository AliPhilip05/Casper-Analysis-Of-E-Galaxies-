fn calculate_c(r_80: f64, r_20: f64) -> f64 {
    if r_20 == 0.0 {
        return 0.0;
    }

    5.0 * (r_80 / r_20).log10()
}

fn estimate_radii(
    flux_array: &[f64],
    shape_xx_arr: &[f64],
    shape_yy_arr: &[f64],
) -> (Vec<f64>, Vec<f64>) {
    let mut r_80_list = Vec::new();
    let mut r_20_list = Vec::new();

    let max_flux = flux_array
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);

    let min_flux = flux_array
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);

    let flux_range = max_flux - min_flux;

    for i in 0..flux_array.len() {
        let flux = flux_array[i];
        let sxx = shape_xx_arr[i];
        let syy = shape_yy_arr[i];

        let r_total = (sxx + syy).abs().sqrt();

        let flux_norm = if flux_range != 0.0 {
            (flux - min_flux) / flux_range
        } else {
            0.5
        };

        let r_80 = r_total * (0.5 + 0.5 * flux_norm);
        let r_20 = r_total * (0.1 + 0.3 * flux_norm);

        r_80_list.push(r_80.max(1e-6));
        r_20_list.push(r_20.max(1e-6));
    }

    (r_80_list, r_20_list)
}

// Rust equivalent of scipy.ndimage.uniform_filter
fn uniform_filter(array: &[f64], size: usize) -> Vec<f64> {
    let mut result = vec![0.0; array.len()];

    if array.is_empty() {
        return result;
    }

    let radius = size / 2;

    for i in 0..array.len() {
        let start = i.saturating_sub(radius);
        let end = (i + radius + 1).min(array.len());

        let sum: f64 = array[start..end].iter().sum();

        result[i] = sum / (end - start) as f64;
    }

    result
}
