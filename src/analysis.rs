use std::collections::HashMap;

/// Represents the graph as an adjacency list.
pub type Graph = HashMap<String, Vec<(String, f64)>>;

/// Represents a mapping of nodes to their community IDs.
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
    graph: &Graph, 
    communities: &CommunityMapping,
) -> f64 {
    let mut modularity = 0.0;
    let mut total_weight = 0.0;

    // Calculate total weight (sum of all edge weights)
    for edges in graph.values() {
        for (_, weight) in edges {
            total_weight += weight;
        }
    }
    total_weight /= 2.0;

    // Compute modularity
    for (node, edges) in graph {
        let node_community = communities.get(node).unwrap_or(&0);
        let degree_i: f64 = edges.iter().map(|(_, weight)| weight).sum();

        for (neighbor, weight) in edges {
            let neighbor_community = communities.get(neighbor).unwrap_or(&0);
            let degree_j: f64 = graph.get(neighbor)
                .map_or(0.0, |neigh_edges| neigh_edges.iter().map(|(_, w)| w).sum());

            if node_community == neighbor_community {
                modularity += *weight - (degree_i * degree_j) / (2.0 * total_weight);
            }
        }
    }

    modularity / (2.0 * total_weight)
}


/// Initializes each node to its own community.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph.
/// 
/// # Returns:
/// - A mapping of nodes to their own community IDs.
pub fn initialize_communities(graph: &Graph) -> CommunityMapping {
    let mut nodes: Vec<_> = graph.keys().collect(); // Collect all node keys
    nodes.sort(); // Sort the nodes lexicographically

    nodes.into_iter() // Iterate over sorted nodes
        .enumerate() // Assign a unique ID (index) to each node
        .map(|(index, node)| (node.clone(), index)) // Map node -> community ID
        .collect() // Collect into a HashMap
}


/// Finds the best community for a node based on modularity gain.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph.
/// - `communities`: Current community assignments.
/// - `total_weight`: Total weight of the graph edges.
/// - `node`: The node to evaluate.
/// 
/// # Returns:
/// - The best community ID for the given node.
pub fn find_best_community_for_node(
    graph: &Graph,
    communities: &CommunityMapping,
    total_weight: f64,
    node: &String,
) -> usize {
    let mut community_weights: HashMap<usize, f64> = HashMap::new();
    let node_degree: f64 = graph.get(node)
        .map_or(0.0, |edges| edges.iter().map(|(_, weight)| weight).sum());

    // Step 1: Sum weights of edges connecting the node to each neighboring community
    if let Some(edges) = graph.get(node) {
        for (neighbor, weight) in edges {
            let neighbor_community = *communities.get(neighbor).unwrap_or(&0);
            *community_weights.entry(neighbor_community).or_insert(0.0) += *weight;
        }
    }

    // Step 2: Evaluate modularity gain for moving to each neighboring community
    let mut best_community = communities.get(node).cloned().unwrap_or(0);
    let mut max_modularity_gain = f64::MIN;

    for (&community, &sigma_in) in &community_weights {
        let sigma_tot: f64 = community_weights.values().sum();
        let modularity_gain = (sigma_in - (node_degree * sigma_tot) / (2.0 * total_weight)) / (2.0 * total_weight);

        if modularity_gain > max_modularity_gain {
            max_modularity_gain = modularity_gain;
            best_community = community;
        }
    }

    best_community
}


/// Updates the degrees of each community based on edge weights.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph.
/// - `communities`: Current community assignments.
/// 
/// # Returns:
/// - A mapping of community IDs to their total degree.
pub fn update_community_degrees(
    graph: &Graph,
    communities: &CommunityMapping
) -> HashMap<usize, f64> {
    let mut community_degrees = HashMap::new();

    for (node, edges) in graph {
        let community = communities.get(node).unwrap_or(&0);
        let degree: f64 = edges.iter().map(|(_, weight)| weight).sum();
        *community_degrees.entry(*community).or_insert(0.0) += degree;
    }

    community_degrees
}

/// Aggregates the graph based on the current community assignments.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph.
/// - `communities`: Mapping of nodes to community IDs.
/// 
/// # Returns:
/// - A new aggregated graph.
/// Aggregates the graph based on the current community assignments.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph.
/// - `communities`: Mapping of nodes to community IDs.
/// 
/// # Returns:
/// - A new aggregated graph.
pub fn aggregate_graph(graph: &Graph, communities: &CommunityMapping) -> Graph {
    let mut aggregated_graph: HashMap<usize, HashMap<usize, f64>> = HashMap::new();

    for (node, edges) in graph {
        let node_community = *communities.get(node).unwrap_or(&0);

        for (neighbor, weight) in edges {
            let neighbor_community = *communities.get(neighbor).unwrap_or(&0);

            *aggregated_graph
                .entry(node_community)
                .or_insert_with(HashMap::new)
                .entry(neighbor_community)
                .or_insert(0.0) += *weight;
        }
    }

    let mut new_graph = Graph::new();
    for (community, edges) in aggregated_graph {
        let edges_vec: Vec<_> = edges
            .into_iter()
            .map(|(neighbor_community, weight)| (neighbor_community.to_string(), weight))
            .collect();

        new_graph.insert(community.to_string(), edges_vec);
    }

    new_graph
}



/// Runs the Louvain algorithm to detect communities.
/// 
/// # Parameters:
/// - `graph`: Reference to the graph.
/// 
/// # Returns:
/// - The final community structure.
pub fn run_louvain(graph: &Graph) -> CommunityMapping {
    let mut communities = initialize_communities(graph);
    let mut current_graph = graph.clone();
    let mut total_weight: f64 = current_graph
        .values()
        .flat_map(|edges| edges.iter())
        .map(|(_, weight)| weight)
        .sum::<f64>() / 2.0;

    let mut previous_modularity = f64::MIN;
    let threshold = 1e-6;

    println!("Initial modularity: {:.6}", previous_modularity);

    loop {
        let mut improved = false;

        for node in current_graph.keys() {
            let best_community = find_best_community_for_node(
                &current_graph,
                &communities,
                total_weight,
                node,
            );

            if communities[node] != best_community {
                communities.insert(node.clone(), best_community);
                improved = true;
            }
        }

        let current_modularity = calculate_modularity(&current_graph, &communities);
        println!("Current modularity: {:.6}", current_modularity);

        if (current_modularity - previous_modularity).abs() < threshold {
            println!("Modularity improvement below threshold. Stopping.");
            break;
        }
        previous_modularity = current_modularity;

        if improved {
            current_graph = aggregate_graph(&current_graph, &communities);
            total_weight = current_graph
                .values()
                .flat_map(|edges| edges.iter())
                .map(|(_, weight)| weight)
                .sum::<f64>() / 2.0;
        } else {
            break;
        }
    }

    println!("Final modularity: {:.6}", previous_modularity);
    communities
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_communities() {
        let mut graph: Graph = HashMap::new();
        graph.insert("A".to_string(), vec![("B".to_string(), 1.0)]);
        graph.insert("B".to_string(), vec![("A".to_string(), 1.0), ("C".to_string(), 2.0)]);
        graph.insert("C".to_string(), vec![("B".to_string(), 2.0)]);

        let communities = initialize_communities(&graph);

        let mut expected = CommunityMapping::new();
        expected.insert("A".to_string(), 0);
        expected.insert("B".to_string(), 1);
        expected.insert("C".to_string(), 2);

        assert_eq!(communities, expected);
    }

    #[test]
    fn test_calculate_modularity() {
        let mut graph: Graph = HashMap::new();
        graph.insert("A".to_string(), vec![("B".to_string(), 1.0)]);
        graph.insert("B".to_string(), vec![("A".to_string(), 1.0), ("C".to_string(), 2.0)]);
        graph.insert("C".to_string(), vec![("B".to_string(), 2.0)]);
    
        let mut communities = CommunityMapping::new();
        communities.insert("A".to_string(), 0);
        communities.insert("B".to_string(), 0);
        communities.insert("C".to_string(), 0); // All nodes in one community
    
        let modularity = calculate_modularity(&graph, &communities);
        println!("the modularity is: {modularity}");
        let expected_modularity = 0.5; // No separation of communities
        assert!((modularity - expected_modularity).abs() < 1e-6);
    }

    #[test]
    fn test_find_best_community_for_node() {
        let mut graph: Graph = HashMap::new();
        graph.insert("A".to_string(), vec![("B".to_string(), 1.0), ("C".to_string(), 1.0)]);
        graph.insert("B".to_string(), vec![("A".to_string(), 1.0)]);
        graph.insert("C".to_string(), vec![("A".to_string(), 1.0)]);

        let mut communities = CommunityMapping::new();
        communities.insert("A".to_string(), 0);
        communities.insert("B".to_string(), 1);
        communities.insert("C".to_string(), 1);

        let total_weight = 3.0; // Total edge weight = 3.0 (1.0 + 1.0 + 1.0)

        let best_community = find_best_community_for_node(&graph, &communities, total_weight, &"A".to_string());

        assert_eq!(best_community, 1);
    }

    #[test]
    fn test_update_community_degrees() {
        let mut graph: Graph = HashMap::new();
        graph.insert("A".to_string(), vec![("B".to_string(), 1.0)]);
        graph.insert("B".to_string(), vec![("A".to_string(), 1.0), ("C".to_string(), 2.0)]);
        graph.insert("C".to_string(), vec![("B".to_string(), 2.0)]);

        let mut communities = CommunityMapping::new();
        communities.insert("A".to_string(), 0);
        communities.insert("B".to_string(), 0);
        communities.insert("C".to_string(), 1);

        let degrees = update_community_degrees(&graph, &communities);

        let mut expected = HashMap::new();
        expected.insert(0, 4.0); // Community 0: A-B (1.0) + B-C (2.0) counted twice for B
        expected.insert(1, 2.0); // Community 1: C-B (2.0)

        assert_eq!(degrees, expected);
    }

    #[test]
    fn test_aggregate_graph() {
        let mut graph: Graph = HashMap::new();
        graph.insert("A".to_string(), vec![("B".to_string(), 1.0)]);
        graph.insert("B".to_string(), vec![("A".to_string(), 1.0), ("C".to_string(), 2.0)]);
        graph.insert("C".to_string(), vec![("B".to_string(), 2.0)]);

        let mut communities = CommunityMapping::new();
        communities.insert("A".to_string(), 0);
        communities.insert("B".to_string(), 0);
        communities.insert("C".to_string(), 1);

        let aggregated = aggregate_graph(&graph, &communities);

        let mut expected = Graph::new();
        expected.insert("0".to_string(), vec![("0".to_string(), 2.0), ("1".to_string(), 2.0)]);
        expected.insert("1".to_string(), vec![("0".to_string(), 2.0)]);

        assert_eq!(aggregated, expected);
    }

}
