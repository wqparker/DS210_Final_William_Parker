use crate::node::Node;
use std::collections::HashMap;

/// Assign edges in HashMaps and return a new structure for storing edges
/// Key: Node ID, Value: Vec<(Index of connected node, edge weight)>
pub fn assign_edges_in_hashmaps(
    maps: &mut [HashMap<String, Node>; 12],
) -> [HashMap<String, Vec<(usize, f64)>>; 12] {
    let mut edge_maps: [HashMap<String, Vec<(usize, f64)>>; 12] = Default::default();

    for (i, map) in maps.iter_mut().enumerate() {
        println!("Processing HashMap {}", i);

        // Collect all nodes in the current HashMap
        let nodes: Vec<_> = map.values().collect();
        let node_count = nodes.len();

        // Create an edge map for the current HashMap
        let mut current_edge_map: HashMap<String, Vec<(usize, f64)>> = HashMap::new();

        for j in 0..node_count {
            let node_a = &nodes[j];

            for k in j + 1..node_count {
                let node_b = &nodes[k];

                // Skip if either node does not have an estimate value
                if node_a.estimate == 0.0 || node_b.estimate == 0.0 {
                    continue;
                }

                // Check if the nodes share at least one matching field
                if node_a.stub_label_num == node_b.stub_label_num
                    || node_a.year_num == node_b.year_num
                    || node_a.age_num == node_b.age_num
                {
                    // Find the higher of the two estimates
                    let higher_estimate = f64::max(node_a.estimate, node_b.estimate);

                    // Normalize the estimates
                    let normalized_a = node_a.estimate / higher_estimate;
                    let normalized_b = node_b.estimate / higher_estimate;

                    // Calculate the edge weight
                    let weight = 1.0 - (normalized_a - normalized_b).abs();

                    // Add bidirectional edges to the edge map
                    current_edge_map
                        .entry(format!(
                            "{}-{}-{}",
                            node_a.stub_label_num, node_a.year_num, node_a.age_num
                        ))
                        .or_default()
                        .push((k, weight));

                    current_edge_map
                        .entry(format!(
                            "{}-{}-{}",
                            node_b.stub_label_num, node_b.year_num, node_b.age_num
                        ))
                        .or_default()
                        .push((j, weight));
                }
            }
        }

        edge_maps[i] = current_edge_map;
    }

    edge_maps
}
