use std::collections::HashMap;
use crate::node::Node;

/// Splits a vector of nodes into two groups based on the `unit_num` value.
/// 
/// # Parameters:
/// - `nodes`: A vector of all nodes.
/// 
/// # Returns:
/// - A tuple containing two vectors:
///   - First vector: Nodes with `unit_num == 1`.
///   - Second vector: Nodes with `unit_num != 1`.
pub fn split_by_unit_num(nodes: Vec<Node>) -> (Vec<Node>, Vec<Node>) {
    let mut group_1 = Vec::new();
    let mut group_2 = Vec::new();

    for node in nodes {
        if node.unit_num == 1 {
            group_1.push(node);
        } else {
            group_2.push(node);
        }
    }

    (group_1, group_2)
}

/// Subdivides a vector of nodes into 11 subvectors based on the `stub_name_num` value.
/// 
/// # Parameters:
/// - `nodes`: A vector of nodes to subdivide.
/// 
/// # Returns:
/// - A vector containing 11 subvectors, each corresponding to a `stub_name_num` value from 0 to 10.
pub fn subdivide_by_stub_name_num(nodes: Vec<Node>) -> Vec<Vec<Node>> {
    let mut groups: Vec<Vec<Node>> = (0..12).map(|_| Vec::new()).collect(); // Initialize 11 empty vectors

    for node in nodes {
        let index = node.stub_name_num as usize;
        if index < 12 {
            groups[index].push(node);
        } else {
            println!("Warning: Node with invalid stub_name_num: {}", node.stub_name_num);
        }
    }

    groups
}


/// Builds a graph from a vector of nodes.
/// 
/// # Parameters:
/// - `nodes`: A vector of nodes to construct the graph.
/// 
/// # Returns:
/// - A `HashMap` where:
///   - Keys are node IDs.
///   - Values are vectors of tuples containing connected node IDs and their respective edge weights.
pub fn build_graph(nodes: Vec<Node>) -> HashMap<String, Vec<(String, f64)>> {
    let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    for (i, node_a) in nodes.iter().enumerate() {
        let node_a_id = format!("{}-{}-{}", node_a.stub_label_num, node_a.year_num, node_a.age_num);

        for node_b in nodes.iter().skip(i + 1) {
            // Skip if either node does not have an estimate value
            if node_a.estimate == 0.0 || node_b.estimate == 0.0 {
                continue;
            }
//
            //let mut counter = 0.0;
            //if node_a.stub_label_num == node_b.stub_label_num {
            //    counter += 1.0;
            //}
            //if node_a.year_num == node_b.year_num {
            //    counter += 1.0;
            //}
            //if node_a.age_num == node_b.age_num {
            //    counter += 1.0;
            //}
            //if counter >= 2.0 {
            // Check if the nodes share at least one matching field
            //if node_a.stub_label_num == node_b.stub_label_num
            //    || node_a.year_num == node_b.year_num
            //    || node_a.age_num == node_b.age_num
            //{
                // Find the higher of the two estimates
                let higher_estimate = f64::max(node_a.estimate, node_b.estimate);

                // Normalize the estimates
                let normalized_a = node_a.estimate / higher_estimate;
                let normalized_b = node_b.estimate / higher_estimate;

                // Calculate the edge weight
                let mut weight = 1.0 - (normalized_a - normalized_b).abs();
                //if node_a.stub_label_num == node_b.stub_label_num {
                //    weight += 1.0;
                //}
                //if node_a.year_num == node_b.year_num {
                //    weight += 1.0;
                //}
                //if node_a.age_num == node_b.age_num {
                //    weight += 1.0;
                //}
                //let weight = weight + counter;
                if weight >= 0.0 {
                    // Add bidirectional edges
                    graph.entry(node_a_id.clone())
                        .or_insert_with(Vec::new)
                        .push((
                            format!("{}-{}-{}", node_b.stub_label_num, node_b.year_num, node_b.age_num),
                            weight,
                        ));
    
                    let node_b_id = format!("{}-{}-{}", node_b.stub_label_num, node_b.year_num, node_b.age_num);
                    graph.entry(node_b_id)
                        .or_insert_with(Vec::new)
                        .push((node_a_id.clone(), weight));
                //}
            }
        }
    }

    graph
}
