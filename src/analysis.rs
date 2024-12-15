use std::collections::{HashMap, HashSet};

/// Represents a mapping of nodes to communities.
pub type CommunityMapping = HashMap<String, usize>;

/// Calculates the modularity of the current partition.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph (node -> [(neighbor, weight)]).
/// - `communities`: Mapping of node IDs to community IDs.
/// 
/// # Returns:
/// - Modularity score (f64).
pub fn calculate_modularity(
    graph: &HashMap<String, Vec<(String, f64)>>,
    communities: &CommunityMapping,
) -> f64 {
    let mut modularity = 0.0;
    let mut total_weight = 0.0;
    let mut community_weights = HashMap::new();

    // Calculate total weight and community weights
    for (node, edges) in graph {
        for (neighbor, weight) in edges {
            total_weight += weight;
            let community = communities.get(node).unwrap_or(&0);
            *community_weights.entry(*community).or_insert(0.0) += weight;
        }
    }

    total_weight /= 2.0; // Each edge is counted twice

    // Calculate modularity
    for (node, edges) in graph {
        let node_community = communities.get(node).unwrap_or(&0);
        for (neighbor, weight) in edges {
            let neighbor_community = communities.get(neighbor).unwrap_or(&0);
            if node_community == neighbor_community {
                let ki = edges.iter().map(|(_, w)| w).sum::<f64>();
                let kj = graph.get(neighbor).unwrap_or(&vec![]).iter().map(|(_, w)| w).sum::<f64>();
                modularity += weight - (ki * kj / (2.0 * total_weight));
            }
        }
    }

    modularity / (2.0 * total_weight)
}

/// Performs the Louvain algorithm to detect communities.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph (node -> [(neighbor, weight)]).
/// 
/// # Returns:
/// - Mapping of node IDs to their final community IDs.
pub fn louvain_algorithm(
    graph: &HashMap<String, Vec<(String, f64)>>,
) -> CommunityMapping {
    let mut communities: CommunityMapping = graph.keys().enumerate().map(|(i, node)| (node.clone(), i)).collect();
    let mut improved = true;
    println!("Beginning base louvain algorithm...");
    let mut improvement_state = 0; // To track the improvement print state

    while improved {
        // Print alternating improvement messages
        match improvement_state % 4 {
            0 => println!("Improving"),
            1 => println!("Improving."),
            2 => println!("Improving.."),
            3 => println!("Improving..."),
            _ => (),
        }
        improvement_state += 1;

        improved = false;

        for node in graph.keys() {
            let current_community = communities.get(node).unwrap_or(&0).to_owned();
            let mut best_community = current_community;
            let mut max_modularity_gain = 0.0;

            let mut community_neighbors: HashMap<usize, f64> = HashMap::new();
            for (neighbor, weight) in &graph[node] {
                let neighbor_community = *communities.get(neighbor).unwrap_or(&0);
                *community_neighbors.entry(neighbor_community).or_insert(0.0) += weight;
            }

            for (&community, &weight) in &community_neighbors {
                let modularity_gain = weight - (weight * weight) / 2.0;
                if modularity_gain > max_modularity_gain {
                    max_modularity_gain = modularity_gain;
                    best_community = community;
                }
            }

            if best_community != current_community {
                communities.insert(node.clone(), best_community);
                improved = true;
            }
        }
    }

    communities
}


/// Aggregates the graph based on the current community assignments.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph (node -> [(neighbor, weight)]).
/// - `communities`: Mapping of node IDs to community IDs.
/// 
/// # Returns:
/// - A new graph aggregated by communities.
pub fn aggregate_graph(
    graph: &HashMap<String, Vec<(String, f64)>>,
    communities: &CommunityMapping,
) -> HashMap<String, Vec<(String, f64)>> {
    let mut aggregated_graph = HashMap::new();

    for (node, edges) in graph {
        let node_community = communities.get(node).unwrap_or(&0).to_string();
        for (neighbor, weight) in edges {
            let neighbor_community = communities.get(neighbor).unwrap_or(&0).to_string();
            if node_community != neighbor_community {
                aggregated_graph.entry(node_community.clone())
                    .or_insert_with(Vec::new)
                    .push((neighbor_community.clone(), *weight));
            }
        }
    }

    aggregated_graph
}

/// Runs the Louvain algorithm and returns the final community structure.
pub fn run_louvain(graph: &HashMap<String, Vec<(String, f64)>>) {
    let mut current_graph = graph.clone();
    let mut communities = louvain_algorithm(&current_graph);

    println!("Initial modularity: {:.4}", calculate_modularity(&current_graph, &communities));

    loop {
        current_graph = aggregate_graph(&current_graph, &communities);
        let new_communities = louvain_algorithm(&current_graph);

        let modularity = calculate_modularity(&current_graph, &new_communities);
        println!("New modularity: {:.4}", modularity);

        if communities == new_communities {
            break;
        }

        communities = new_communities;
    }

    println!("Final community structure: {:?}", communities);
}
