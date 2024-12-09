use std::collections::{HashMap, HashSet};
use crate::node::{Node, Edge};

pub fn louvain_algorithm(graph: &HashMap<String, Node>) -> Vec<HashSet<String>> {
    // Step 1: Initialize each node as its own community
    let mut communities: HashMap<String, HashSet<String>> = HashMap::new();
    for node_id in graph.keys() {
        let mut community = HashSet::new();
        community.insert(node_id.clone());
        communities.insert(node_id.clone(), community);
    }

    // Step 2: Track modularity changes
    let mut modularity_improved = true;

    while modularity_improved {
        modularity_improved = false;

        // Phase 1: Local Optimization
        for (node_id, node) in graph.iter() {
            // Calculate the best community for the node
            let current_modularity = calculate_modularity();
            let mut best_modularity = current_modularity;
            let mut best_community = None;

            for (community_id, community) in &communities {
                if community.contains(node_id) {
                    continue; // Skip the current community
                }

                // Temporarily move the node to the new community
                community.insert(node_id.clone());
                let new_modularity = calculate_modularity();

                if new_modularity > best_modularity {
                    best_modularity = new_modularity;
                    best_community = Some(community_id.clone());
                }

                // Revert the change
                community.remove(node_id);
            }

            // Move the node if it improves modularity
            if let Some(new_community_id) = best_community {
                communities.get_mut(&new_community_id).unwrap().insert(node_id.clone());
                communities.get_mut(node_id).unwrap().remove(node_id);
                modularity_improved = true;
            }
            // Move the node to the best community if modularity improves
        }

        // Phase 2: Community Aggregation
        // Aggregate communities into super-nodes and create a new graph
        let mut new_graph: HashMap<String, Node> = HashMap::new();

        for (community_id, community) in &communities {
            let mut aggregated_node = Node::new(); // Replace with actual initialization of Node

            for node_id in community {
                let node = graph.get(node_id).unwrap();
                for edge in &node.edges {
                    if community.contains(&edge.target) {
                        aggregated_node.add_internal_edge(edge.weight); // Adjust as needed
                    } else {
                        aggregated_node.add_external_edge(edge.target.clone(), edge.weight); // Adjust as needed
                    }
                }
            }

            new_graph.insert(community_id.clone(), aggregated_node);
        }

        // Replace graph with the new aggregated graph
        *graph = new_graph;

        // Update modularity_improved based on changes in modularity
    }

    // Convert the final communities HashMap to a Vec<HashSet<String>> for output
    let mut result = Vec::new();
    for (_, community) in communities {
        result.push(community);
    }

    result
}

// TODO: Add helper functions for modularity calculation and aggregation
fn calculate_modularity(graph: &HashMap<String, Node>, communities: &HashMap<String, HashSet<String>>) -> f64 {
    let mut modularity = 0.0;
    let total_weight: f64 = graph.values().flat_map(|node| node.edges.iter().map(|edge| edge.weight)).sum();

    for (community_id, community) in communities {
        let mut internal_weight = 0.0;
        let mut total_community_weight = 0.0;

        for node_id in community {
            if let Some(node) = graph.get(node_id) {
                total_community_weight += node.edges.iter().map(|edge| edge.weight).sum::<f64>();
                internal_weight += node.edges.iter()
                    .filter(|edge| community.contains(&edge.target))
                    .map(|edge| edge.weight)
                    .sum::<f64>();
            }
        }

        let fraction = total_community_weight / total_weight;
        modularity += (internal_weight / total_weight) - (fraction * fraction);
    }

    modularity
}

fn aggregate_communities(graph: &HashMap<String, Node>, communities: &HashMap<String, HashSet<String>>) -> HashMap<String, Node> {
    let mut new_graph: HashMap<String, Node> = HashMap::new();

    for (community_id, community) in communities {
        let mut aggregated_node = Node::new(); // Replace with actual initialization of Node

        for node_id in community {
            if let Some(node) = graph.get(node_id) {
                for edge in &node.edges {
                    if community.contains(&edge.target) {
                        aggregated_node.add_internal_edge(edge.weight); // Adjust as needed
                    } else {
                        aggregated_node.add_external_edge(edge.target.clone(), edge.weight); // Adjust as needed
                    }
                }
            }
        }

        new_graph.insert(community_id.clone(), aggregated_node);
    }

    new_graph
}
