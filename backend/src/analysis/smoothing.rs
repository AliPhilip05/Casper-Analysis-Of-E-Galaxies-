/// Apply a moving-average filter, shrinking its window at the array edges.
pub fn uniform_filter(array: &[f64], size: usize) -> Vec<f64> {
    let mut result = vec![0.0; array.len()];
    if array.is_empty() {
        return result;
    }

    let radius = size / 2;
    for (index, value) in result.iter_mut().enumerate() {
        let start = index.saturating_sub(radius);
        let end = (index + radius + 1).min(array.len());
        let sum: f64 = array[start..end].iter().sum();
        *value = sum / (end - start) as f64;
    }

    result
}
