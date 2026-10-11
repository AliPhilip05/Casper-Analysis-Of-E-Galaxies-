/// Calculate concentration from the estimated 80% and 20% radii.
pub fn calculate_c(r_80: f64, r_20: f64) -> f64 {
    if r_20 == 0.0 {
        return 0.0;
    }

    5.0 * (r_80 / r_20).log10()
}
