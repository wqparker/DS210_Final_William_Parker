use crate::node::{Edge, Node};
use std::collections::HashMap;

pub fn assign_edges_in_hashmaps(maps: &mut [HashMap<String, Node>; 12]) {
    for (i, map) in maps.iter_mut().enumerate() {
        println!("Processing HashMap {}", i);

        // Collect all nodes in the current HashMap
        let mut nodes: Vec<_> = map.values_mut().collect();

        let node_count = nodes.len();
        for j in 0..node_count {
            let (left, right) = nodes.split_at_mut(j + 1);
            let node_a = &mut left[j];

            for node_b in right.iter_mut() {
                // skip if either node does not have an estimate value
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

                    // Create edges
                    let edge_a_to_b = Edge {
                        target: format!(
                            "{}-{}-{}",
                            node_b.stub_label_num, node_b.year_num, node_b.age_num
                        ),
                        weight,
                    };
                    let edge_b_to_a = Edge {
                        target: format!(
                            "{}-{}-{}",
                            node_a.stub_label_num, node_a.year_num, node_a.age_num
                        ),
                        weight,
                    };

                    // Add bidirectional edges
                    node_a.edges.push(edge_a_to_b);
                    node_b.edges.push(edge_b_to_a);
                }
            }
        }
    }
}
