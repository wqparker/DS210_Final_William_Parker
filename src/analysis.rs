use std::collections::HashMap;

/// Represents the graph as an adjacency list.
pub type Graph = HashMap<String, Vec<(String, f64)>>;

/// Finds the most frequent label among a node's neighbors.
/// 
/// # Parameters:
/// - `neighbors`: A list of neighbors.
/// - `labels`: Reference to the current node-to-label mapping.
///
/// # Returns:
/// - The most frequent label among neighbors.
pub fn most_frequent_label(
    neighbors: &Vec<(String, f64)>,
    labels: &HashMap<String, String>,
) -> Option<String> {
    let mut label_weights: HashMap<String, f64> = HashMap::new();

    // Accumulate weights for each label
    for (neighbor, weight) in neighbors {
        if let Some(label) = labels.get(neighbor) {
            *label_weights.entry(label.clone()).or_insert(0.0) += *weight;
        }
    }

    // Find the label with the maximum cumulative weight
    label_weights
        .into_iter()
        .max_by(|(_, weight_a), (_, weight_b)| weight_a.partial_cmp(weight_b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(label, _)| label)
}

/// Propagates labels to update each node's label based on its neighbors.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph.
/// - `labels`: Mutable reference to the current node-to-label mapping.
///
/// # Returns:
/// - Whether any labels changed in this iteration.
pub fn propagate_labels(graph: &Graph, labels: &mut HashMap<String, String>) -> bool {
    let mut labels_changed = false;

    // Iterate over each node in the graph
    for (node, neighbors) in graph {
        if let Some(new_label) = most_frequent_label(neighbors, labels) {
            if labels.get(node).unwrap() != &new_label {
                labels.insert(node.clone(), new_label);
                labels_changed = true;
            }
        }
    }

    labels_changed
}

/// Runs the label propagation algorithm.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph.
///
/// # Returns:
/// - Final node-to-cluster mapping.
pub fn run_label_propagation(graph: &Graph) -> HashMap<String, String> {
    // Initialize labels: each node starts with its own name as its label
    let mut labels: HashMap<String, String> = graph
        .keys()
        .map(|node| (node.clone(), node.clone()))
        .collect();

    let mut iterations = 0;
    let max_iterations = 100; // Prevent infinite loops

    loop {
        iterations += 1;
        let labels_changed = propagate_labels(graph, &mut labels);

        println!("Iteration {}: {} labels updated", iterations, labels_changed);

        if !labels_changed || iterations >= max_iterations {
            break;
        }
    }

    labels
}
