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
        ("scientific", vec!["simulation", "model", "analysis", "experiment"]),
        ("optimization", vec!["efficiency", "cost", "resource", "scheduling"]),
        ("ai", vec!["machine learning", "neural network", "nlp", "computer vision"]),
    ];

    let matches = keywords.iter()
        .map(|(domain, keywords)| (domain, keywords.iter().any(|kw| problem.description.contains(kw))))
        .filter(|(_, matches)| *matches)
        .collect::<Vec<_>>();

    // 2. NLP-Based Categorization (optional):
    // Could use a library like rust-bert or similar for more sophisticated analysis

    // 3. Combine Scores:
    let domain_score = matches.iter()
        .map(|(domain, _)| match domain {
            &"scientific" => 0.8, // Example weights, adjust as needed
            &"optimization" => 0.6,
            &"ai" => 0.4, 
            _ => 0.0 
        }).sum::<f64>();

    domain_score / matches.len() as f64 // Normalize by number of matches
}

fn calculate_real_world_impact(problem: &UsefulWorkProblem) -> f64 {
    // 1. Domain-Specific Metrics:
    // For scientific, look at potential citations, publications in top journals.
    // For optimization, estimate cost savings or efficiency gains based on problem data.
    // For AI, assess accuracy improvement, processing speedup, etc.
    // ... (Implement logic here) ...

    // 2. Use Standardized Scales:
    // Example: If using a Likert scale (1-5), could have a lookup table based on domain-specific metric values.

    // Placeholder return for now, replace with actual calculation
    0.0 // Replace with real calculation
}

fn calculate_community_demand(problem: &UsefulWorkProblem) -> f64 {
    // 1. Utilize Feedback System:
    // let feedback = Feedback::get_feedback_for_problem(problem.id);
    // Analyze feedback (votes, comments, etc.)

    // 2. Social Media Engagement:
    // Optionally, use APIs to gauge mentions, discussions related to the problem.

    // Placeholder return
    0.0
}

fn calculate_data_availability(problem: &UsefulWorkProblem) -> f64 {
    // 1. Check Known Repositories:
    // Query `DataRepository` (placeholder) for datasets matching problem description.

    // 2. Assess Dataset Quality:
    // Consider size, completeness, documentation, format, etc.

    // Placeholder return
    0.0
}
