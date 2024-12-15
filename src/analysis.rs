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

    // Calculate total edge weight for modularity calculations
    let total_weight: f64 = graph.values()
        .flat_map(|edges| edges.iter())
        .map(|(_, w)| *w)
        .sum::<f64>() / 2.0; // Each edge is counted twice

    // Precompute initial community degrees
    let mut community_degrees: HashMap<usize, f64> = HashMap::new();
    for (node, edges) in graph {
        let community = *communities.get(node).unwrap_or(&0);
        let degree: f64 = edges.iter().map(|(_, weight)| weight).sum();
        *community_degrees.entry(community).or_insert(0.0) += degree;
    }

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
    
        println!("Modularity before iteration {}: {:.6}", improvement_state, calculate_modularity(graph, &communities));
    
        improved = false;
    
        for node in graph.keys() {
            let current_community = communities.get(node).unwrap_or(&0).to_owned();
            let mut best_community = current_community;
            let mut max_modularity_gain = 0.0;
    
            let mut community_neighbors: HashMap<usize, f64> = HashMap::new();
            let node_degree: f64 = graph[node].iter().map(|(_, weight)| weight).sum();
    
            // Sum weights of edges connecting to each community
            for (neighbor, weight) in &graph[node] {
                let neighbor_community = *communities.get(neighbor).unwrap_or(&0);
                *community_neighbors.entry(neighbor_community).or_insert(0.0) += weight;
            }
    
            // Evaluate modularity gain for moving node to each neighboring community
            for (&community, &edge_sum) in &community_neighbors {
                let community_degree = community_degrees.get(&community).unwrap_or(&0.0);
                let modularity_gain = (edge_sum / total_weight) - (node_degree * community_degree) / (2.0 * total_weight * total_weight);
    
                if modularity_gain > max_modularity_gain {
                    max_modularity_gain = modularity_gain;
                    best_community = community;
                }
            }
    
            // Move node to the best community and update degrees
            if best_community != current_community {
                communities.insert(node.clone(), best_community);
                *community_degrees.entry(current_community).or_insert(0.0) -= node_degree;
                *community_degrees.entry(best_community).or_insert(0.0) += node_degree;
                improved = true;
            }
        }
    
        println!("Modularity after iteration {}: {:.6}", improvement_state, calculate_modularity(graph, &communities));
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
    communities: &CommunityMapping
) -> HashMap<String, Vec<(String, f64)>> {
    let mut aggregated_graph: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for (node, edges) in graph {
        let node_community = communities.get(node).unwrap_or(&0).to_string();
        for (neighbor, weight) in edges {
            let neighbor_community = communities.get(neighbor).unwrap_or(&0).to_string();

            // Merge edges by summing weights between communities
            *aggregated_graph
                .entry(node_community.clone())
                .or_insert_with(HashMap::new)
                .entry(neighbor_community.clone())
                .or_insert(0.0) += weight;
        }
    }

    // Convert to the expected output format
    let mut new_graph = HashMap::new();
    for (community, neighbors) in aggregated_graph {
        new_graph.insert(
            community,
            neighbors.into_iter().collect::<Vec<(String, f64)>>(),
        );
    }

    new_graph
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
