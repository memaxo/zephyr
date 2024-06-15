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
            let distance = vec1.iter()
                .zip(vec2)
                .filter(|(x, y)| (x.signum() - y.signum()).abs() > f64::EPSILON)
                .count();
            distance
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
    PearsonCorrelation,
    SpearmanRankCorrelation,
    KullbackLeiblerDivergence,
    MahalanobisDistance,
}
pub fn pearson_correlation(vec1: &[f64], vec2: &[f64]) -> f64 {
    let n = vec1.len();
    let sum1: f64 = vec1.iter().sum();
    let sum2: f64 = vec2.iter().sum();
    let sum1_sq: f64 = vec1.iter().map(|&x| x * x).sum();
    let sum2_sq: f64 = vec2.iter().map(|&x| x * x).sum();
    let p_sum: f64 = vec1.iter().zip(vec2).map(|(&x, &y)| x * y).sum();
    let num = p_sum - (sum1 * sum2 / n as f64);
    let den = ((sum1_sq - sum1.powi(2) / n as f64) * (sum2_sq - sum2.powi(2) / n as f64)).sqrt();
    if den == 0.0 { 0.0 } else { num / den }
}

pub fn spearman_rank_correlation(vec1: &[f64], vec2: &[f64]) -> f64 {
    let ranks1 = rank(vec1);
    let ranks2 = rank(vec2);
    pearson_correlation(&ranks1, &ranks2)
}

fn rank(vec: &[f64]) -> Vec<f64> {
    let mut sorted = vec.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec.iter().map(|x| sorted.binary_search(x).unwrap() as f64 + 1.0).collect()
}

pub fn kullback_leibler_divergence(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter().zip(vec2).map(|(&x, &y)| x * (x / y).ln()).sum()
}

pub fn mahalanobis_distance(vec1: &[f64], vec2: &[f64], cov_inv: &[f64]) -> f64 {
    let diff: Vec<f64> = vec1.iter().zip(vec2).map(|(&x, &y)| x - y).collect();
    let mut sum = 0.0;
    for i in 0..diff.len() {
        for j in 0..diff.len() {
            sum += diff[i] * cov_inv[i * diff.len() + j] * diff[j];
        }
    }
    sum.sqrt()
}
