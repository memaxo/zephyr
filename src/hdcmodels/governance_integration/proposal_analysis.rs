use crate::governance::Proposal;
use crate::hdcmodels::encoding::{encode_natural_language, encode_rust_code};
use crate::hdcmodels::similarity::cosine_similarity;
use crate::hdcmodels::HDCModel;

pub struct ProposalAnalyzer {
    hdc_model: HDCModel,
}

impl ProposalAnalyzer {
    pub fn new(hdc_model: HDCModel) -> Self {
        ProposalAnalyzer { hdc_model }
    }

    pub fn analyze_proposal(&self, proposal: &Proposal) -> ProposalAnalysis {
        let description_vector = self.encode_proposal_description(&proposal.description);
        let code_vector = self.encode_proposal_code(&proposal.code);

        let similarity = cosine_similarity(&description_vector, &code_vector);

        ProposalAnalysis {
            description_vector,
            code_vector,
            similarity,
        }
    }

    pub fn summarize_proposal(&self, proposal: &Proposal) -> String {
        // Encode the proposal description
        let description_vector = self.encode_proposal_description(&proposal.description);

        // Encode the proposal code
        let code_vector = self.encode_proposal_code(&proposal.code);

        // Compute the similarity between the description and code vectors
        let similarity = cosine_similarity(&description_vector, &code_vector);

        // Generate a summary based on the similarity score
        let summary = if similarity >= 0.8 {
            format!("The proposal '{}' has a high similarity between its description and code. It aims to {}.", proposal.title, proposal.description)
        } else if similarity >= 0.6 {
            format!("The proposal '{}' has a moderate similarity between its description and code. It proposes to {}.", proposal.title, proposal.description)
        } else {
            format!("The proposal '{}' has a low similarity between its description and code. It suggests {}.", proposal.title, proposal.description)
        };

        summary
    }

    pub fn compare_proposals(&self, proposal1: &Proposal, proposal2: &Proposal) -> f64 {
        let analysis1 = self.analyze_proposal(proposal1);
        let analysis2 = self.analyze_proposal(proposal2);

        cosine_similarity(&analysis1.description_vector, &analysis2.description_vector)
    }

    pub fn cluster_proposals(&self, proposals: &[Proposal]) -> Vec<Vec<Proposal>> {
        // Convert proposals to feature vectors
        let mut proposal_vectors: Vec<(usize, Vec<f64>)> = proposals
            .iter()
            .enumerate()
            .map(|(i, proposal)| {
                let description_vector = self.encode_proposal_description(&proposal.description);
                let code_vector = self.encode_proposal_code(&proposal.code);
                let combined_vector: Vec<f64> = description_vector
                    .iter()
                    .chain(code_vector.iter())
                    .cloned()
                    .collect();
                (i, combined_vector)
            })
            .collect();

        // Perform k-means clustering
        let k = 3; // Number of clusters
        let max_iterations = 100;
        let mut centroids = Vec::new();

        // Initialize centroids randomly
        for _ in 0..k {
            let random_index = rand::thread_rng().gen_range(0..proposal_vectors.len());
            let random_vector = proposal_vectors[random_index].1.clone();
            centroids.push(random_vector);
        }

        for _ in 0..max_iterations {
            let mut clusters: Vec<Vec<usize>> = vec![Vec::new(); k];

            // Assign proposals to the nearest centroid
            for (i, vector) in &proposal_vectors {
                let mut min_distance = f64::MAX;
                let mut nearest_centroid_index = 0;

                for (j, centroid) in centroids.iter().enumerate() {
                    let distance = cosine_similarity(vector, centroid);
                    if distance < min_distance {
                        min_distance = distance;
                        nearest_centroid_index = j;
                    }
                }

                clusters[nearest_centroid_index].push(*i);
            }

            // Update centroids
            for (j, cluster) in clusters.iter().enumerate() {
                let mut sum_vector = vec![0.0; proposal_vectors[0].1.len()];
                for &i in cluster {
                    for (d, &value) in proposal_vectors[i].1.iter().enumerate() {
                        sum_vector[d] += value;
                    }
                }
                for d in 0..sum_vector.len() {
                    sum_vector[d] /= cluster.len() as f64;
                }
                centroids[j] = sum_vector;
            }
        }

        // Convert cluster indices to actual proposals
        let clustered_proposals: Vec<Vec<Proposal>> = clusters
            .into_iter()
            .map(|cluster| cluster.into_iter().map(|i| proposals[i].clone()).collect())
            .collect();

        clustered_proposals
    }

    fn encode_proposal_description(&self, description: &str) -> Vec<f64> {
        encode_natural_language(description, self.hdc_model.dimension)
    }

    fn encode_proposal_code(&self, code: &str) -> Vec<f64> {
        encode_rust_code(code, self.hdc_model.dimension)
    }
}

pub struct ProposalAnalysis {
    description_vector: Vec<f64>,
    code_vector: Vec<f64>,
    similarity: f64,
}
