use std::collections::HashMap;
use crate::node::Node;

// splits the base vector of all nodes in data into 2 base subgroups
// division based on crude or adjusted numbers
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

// subdivides a group of nodes into their sub categories based on how they categorize demographics
// returns a vector of vectors of the subdivided nodes
pub fn subdivide_by_stub_name_num(nodes: Vec<Node>) -> Vec<Vec<Node>> {
    let mut groups: Vec<Vec<Node>> = (0..12).map(|_| Vec::new()).collect(); // Initialize 11 empty vectors
    // iterate across all nodes and assign a group
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


// build a graph of connected nodes with weighted edges
// takes in a single vector of nodes, outputs a connected hashmap representing a connect, weighted graph
pub fn build_graph(nodes: Vec<Node>) -> HashMap<String, Vec<(String, f64)>> {
    let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    let mut edge_counter = 0;
    // iterate through all nodes
    for (i, node_a) in nodes.iter().enumerate() {
        let node_a_id = format!("{}-{}-{}", node_a.stub_label_num, node_a.year_num, node_a.age_num);
        // iterate through all subsequent nodes
        for node_b in nodes.iter().skip(i + 1) {
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

                // if eight exists, assign logic
                if weight >= 0.0 {
                    edge_counter += 2;
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
                }
            }
        }
    }
    // debugging line
    println!("This graph has {edge_counter} edges");

    graph
}

// sort a graph, used to ensure tests work properly
fn sort_graph(graph: &mut HashMap<String, Vec<(String, f64)>>) -> Vec<(String, Vec<(String, f64)>)> {
    let mut sorted_graph: Vec<_> = graph.iter()
        .map(|(key, value)| {
            let mut sorted_edges = value.clone();
            sorted_edges.sort_by(|a, b| a.0.cmp(&b.0)); // Sort edges by node ID
            (key.clone(), sorted_edges)
        })
        .collect();

    sorted_graph.sort_by(|a, b| a.0.cmp(&b.0)); // Sort top-level keys
    sorted_graph
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;
    use std::collections::HashMap;

    #[test]
    fn test_build_graph() {
        // Input test data: Create nodes
        let nodes = vec![
            Node::new("Deaths per 100,000", 1, "Sex and race", 4, "Male: AI/AN", 4.13, "1988", 12, "All ages", 0, 20.2),
            Node::new("Deaths per 100,000", 1, "Sex and race", 4, "Male: AI/AN", 4.13, "1989", 13, "All ages", 0, 19.9),
            Node::new("Deaths per 100,000", 2, "Sex and race", 4, "Male: AI/AN", 4.13, "1990", 14, "All ages", 0, 20.9),
        ];

        // Expected output graph
        let mut expected_graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        // Manually construct the expected graph
        expected_graph.insert(
            "4.13-12-0".to_string(),
            vec![
                ("4.13-13-0".to_string(), 1.0_f64 - (20.2_f64 / 20.2_f64 - 19.9_f64 / 20.2_f64).abs()),
                ("4.13-14-0".to_string(), 1.0_f64 - (20.2_f64 / 20.9_f64 - 20.9_f64 / 20.9_f64).abs()),
            ],
        );

        expected_graph.insert(
            "4.13-13-0".to_string(),
            vec![
                ("4.13-12-0".to_string(), 1.0_f64 - (19.9_f64 / 20.2_f64 - 20.2_f64 / 20.2_f64).abs()),
                ("4.13-14-0".to_string(), 1.0_f64 - (19.9_f64 / 20.9_f64 - 20.9_f64 / 20.9_f64).abs()),
            ],
        );

        expected_graph.insert(
            "4.13-14-0".to_string(),
            vec![
                ("4.13-12-0".to_string(), 1.0_f64 - (20.9_f64 / 20.9_f64 - 20.2_f64 / 20.9_f64).abs()),
                ("4.13-13-0".to_string(), 1.0_f64 - (20.9_f64 / 20.9_f64 - 19.9_f64 / 20.9_f64).abs()),
            ],
        );

        // Call the function to build the graph
        let mut graph = build_graph(nodes);

        // Sort both graphs
        let sorted_actual = sort_graph(&mut graph);
        let sorted_expected = sort_graph(&mut expected_graph);

        assert_eq!(sorted_actual, sorted_expected);
    }
}
