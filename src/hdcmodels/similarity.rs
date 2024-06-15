pub fn cosine_similarity(vec1: &[f64], vec2: &[f64], metric: SimilarityMetric) -> f64 {
    match metric {
        SimilarityMetric::CosineSimilarity => {
    let dot_product = vec1.iter().zip(vec2.iter()).map(|(x, y)| x * y).sum::<f64>();
    let magnitude1 = vec1.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let magnitude2 = vec2.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();

    dot_product / (magnitude1 * magnitude2)
}

pub fn calculate_euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter()
        .zip(vec2)
        .map(|(x1, x2)| (x1 - x2).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn calculate_manhattan_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter()
        .zip(vec2)
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum::<f64>()
}

pub fn calculate_jaccard_similarity(vec1: &[u8], vec2: &[u8]) -> f64 {
    let intersection = vec1.iter().zip(vec2).filter(|&(x1, x2)| *x1 == 1 && *x2 == 1).count();
    let union = vec1.iter().zip(vec2).filter(|&(x1, x2)| *x1 == 1 || *x2 == 1).count();
    intersection as f64 / union as f64
}

pub fn calculate_pearson_correlation(vec1: &[f64], vec2: &[f64]) -> f64 {
    let n = vec1.len();
    let sum_x = vec1.iter().sum::<f64>();
    let sum_y = vec2.iter().sum::<f64>();
    let sum_xx = vec1.iter().map(|x| x * x).sum::<f64>();
    let sum_yy = vec2.iter().map(|y| y * y).sum::<f64>();
    let sum_xy = vec1.iter().zip(vec2).map(|(x, y)| x * y).sum::<f64>();
    let numerator = n as f64 * sum_xy - sum_x * sum_y;
    let denominator = ((n as f64 * sum_xx - sum_x * sum_x) * (n as f64 * sum_yy - sum_y * sum_y)).sqrt();
    numerator / denominator
}

        }
        _ => panic!("Invalid similarity metric for cosine similarity"),
    }
}

pub fn hamming_distance(vec1: &[f64], vec2: &[f64], metric: SimilarityMetric) -> usize {
    match metric {
        SimilarityMetric::HammingDistance => {
    vec1.iter()
        .zip(vec2)
        .filter(|(x, y)| (x.signum() - y.signum()).abs() > f64::EPSILON)
        .count()
}
        }
        _ => panic!("Invalid similarity metric for Hamming distance"),
    }
}

pub fn jaccard_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let intersection = vec1.iter().zip(vec2).filter(|&(x, y)| (x - y).abs() < f64::EPSILON).count();
    let union = vec1.len() + vec2.len() - intersection;
    intersection as f64 / union as f64
}

pub fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter()
        .zip(vec2)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub enum SimilarityMetric {
    CosineSimilarity,
    HammingDistance,
    JaccardSimilarity,
    EuclideanDistance,
}
