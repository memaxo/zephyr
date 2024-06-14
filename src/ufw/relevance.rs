pub fn calculate_relevance(problem: &UsefulWorkProblem) -> u32 {
    let domain_alignment = calculate_domain_alignment(problem);
    let real_world_impact = calculate_real_world_impact(problem);
    let community_demand = calculate_community_demand(problem);
    let data_availability = calculate_data_availability(problem);

    domain_alignment + real_world_impact + community_demand + data_availability
}

fn calculate_domain_alignment(problem: &UsefulWorkProblem) -> f64 {
    // 1. Keyword/Attribute Matching
    let keywords = vec![
        ("scientific", vec!["simulation", "model", "analysis", "experiment", "quantum", "cryptography"]),
        ("optimization", vec!["efficiency", "cost", "resource", "scheduling", "supply chain", "logistics"]),
        ("ai", vec!["machine learning", "neural network", "nlp", "computer vision", "reinforcement learning", "generative models", "explainable AI"]),
        ("engineering", vec!["design", "simulation", "optimization", "construction", "material"]),
        ("healthcare", vec!["diagnosis", "treatment", "drug discovery", "medical imaging", "patient care"]),
        ("sustainability", vec!["renewable energy", "carbon footprint", "climate change", "pollution", "waste management"]),
        ("finance", vec!["risk assessment", "portfolio optimization", "fraud detection", "algorithmic trading"]),
    ];

    let matches = keywords.iter()
        .map(|(domain, keywords)| (domain, keywords.iter().any(|kw| problem.description.contains(kw))))
        .filter(|(_, matches)| *matches)
        .collect::<Vec<_>>();

    // 2. NLP-Based Categorization:
    let nlp_score = calculate_nlp_score(problem);

    // 3. Combine Scores:
    let domain_score = matches.iter()
        .map(|(domain, _)| match domain {
            &"scientific" => 0.8, // Example weights, adjust as needed
            &"optimization" => 0.6,
            &"ai" => 0.4, 
            _ => 0.0 
        }).sum::<f64>();

    (domain_score / matches.len() as f64) + nlp_score // Combine keyword and NLP scores
}

fn calculate_real_world_impact(problem: &UsefulWorkProblem) -> f64 {
    // 1. Domain-Specific Metrics:
    // For scientific, look at potential citations, publications in top journals.
    // For optimization, estimate cost savings or efficiency gains based on problem data.
    // For AI, assess accuracy improvement, processing speedup, etc.
    // ... (Implement logic here) ...

    // 2. Use Standardized Scales:
    // Example: If using a Likert scale (1-5), could have a lookup table based on domain-specific metric values.

    // Example implementation for scientific domain
    if problem.domain == "scientific" {
        let potential_citations = estimate_citations(problem);
        let publication_impact_factor = estimate_publication_impact_factor(problem);
        let potential_patents = estimate_potential_patents(problem);
        return (potential_citations + publication_impact_factor + potential_patents) / 3.0;
    }
    // Implement similar logic for other domains
    0.0
}

fn calculate_community_demand(problem: &UsefulWorkProblem) -> f64 {
    // 1. Utilize Feedback System:
    // let feedback = Feedback::get_feedback_for_problem(problem.id);
    // Analyze feedback (votes, comments, etc.)

    // 2. Social Media Engagement:
    // Optionally, use APIs to gauge mentions, discussions related to the problem.

    // Example implementation for feedback system
    let feedback_score = get_feedback_score(problem);
    let social_media_score = get_social_media_score(problem);
    (feedback_score + social_media_score) / 2.0
}

fn calculate_data_availability(problem: &UsefulWorkProblem) -> f64 {
    // 1. Check Known Repositories:
    // Query `DataRepository` (placeholder) for datasets matching problem description.

    // 2. Assess Dataset Quality:
    // Consider size, completeness, documentation, format, etc.

    // Example implementation for data availability
    let repository_score = check_repositories(problem);
    let quality_score = assess_dataset_quality(problem);
    (repository_score + quality_score) / 2.0
}
