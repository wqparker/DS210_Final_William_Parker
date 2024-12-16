use std::collections::HashMap;

// Represents the graph as an adjacency list.
pub type Graph = HashMap<String, Vec<(String, f64)>>;

// find the most frequent labels for a given node's neightbors
pub fn most_frequent_label(
    neighbors: &Vec<(String, f64)>,
    labels: &HashMap<String, String>,
) -> Vec<(String, f64)> {
    let mut label_weights: HashMap<String, f64> = HashMap::new();

    // Sum weights for each label
    for (neighbor, weight) in neighbors {
        if let Some(label) = labels.get(neighbor) {
            *label_weights.entry(label.clone()).or_insert(0.0) += weight;
        }
    }

    // Find the maximum weight
    let max_weight = label_weights.values().cloned().fold(f64::MIN, f64::max);

    // Collect labels with the maximum weight
    label_weights
        .into_iter()
        .filter(|&(_, weight)| weight == max_weight)
        .collect()
}

// propagate the labels
pub fn propagate_labels(graph: &Graph, labels: &mut HashMap<String, String>) -> bool {
    let mut updated = false;
    let mut new_labels = labels.clone();

    for (node, neighbors) in graph {
        // Get the most frequent label(s)
        if let Some((new_label, _)) = most_frequent_label(neighbors, labels).get(0) {
            if labels[node] != *new_label {
                new_labels.insert(node.clone(), new_label.clone());
                updated = true;
            }
        }
    }

    *labels = new_labels; // Update the labels
    updated
}

// wrapper function to iterate through graph until it has reduced down to just a handfull of nodes
pub fn run_label_propagation(graph: &Graph) -> Graph {
    // Initialize labels: each node starts with its own name as the label
    let mut labels: HashMap<String, String> = graph
        .keys()
        .map(|node| (node.clone(), node.clone()))
        .collect();

    // Iterate until no labels are updated
    while propagate_labels(graph, &mut labels) {
        println!("Propagating labels...");
    }

    // Construct the final graph with updated labels
    let mut new_graph: Graph = HashMap::new();

    for (node, neighbors) in graph {
        let node_label = labels.get(node).unwrap().clone();
        let updated_neighbors: Vec<(String, f64)> = neighbors
            .iter()
            .map(|(neighbor, weight)| {
                let neighbor_label = labels.get(neighbor).unwrap().clone();
                (neighbor_label, *weight)
            })
            .collect();

        new_graph.insert(node_label, updated_neighbors);
    }

    new_graph
}
