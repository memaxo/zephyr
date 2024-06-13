pub fn cosine_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let dot_product = vec1.iter().zip(vec2.iter()).map(|(x, y)| x * y).sum::<f64>();
    let magnitude1 = vec1.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let magnitude2 = vec2.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();

    dot_product / (magnitude1 * magnitude2)
}

pub fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn jaccard_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let intersection: f64 = vec1
        .iter()
        .zip(vec2.iter())
        .filter(|(x, y)| x.signum() == y.signum())
        .count() as f64;

    let union: f64 = vec1.len() as f64 + vec2.len() as f64 - intersection;

    intersection / union
}

pub fn hamming_distance(vec1: &[f64], vec2: &[f64]) -> usize {
    let dot_product = vec1.iter().zip(vec2.iter()).map(|(x, y)| x * y).sum::<f64>();
    let magnitude1 = vec1.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let magnitude2 = vec2.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();

    dot_product / (magnitude1 * magnitude2)
}

pub fn hamming_distance(vec1: &[f64], vec2: &[f64]) -> usize {
    vec1.iter()
        .zip(vec2.iter())
        .filter(|(x, y)| (x.signum() - y.signum()).abs() > f64::EPSILON)
        .count()
}
