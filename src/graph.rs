use std::collections::HashMap;
use crate::node::Node;

pub type Graph = HashMap<String, Node>; // A graph of nodes


pub fn group_nodes_into_graphs(nodes: Vec<Node>) -> [Graph; 11] {
    // Create an array of 11 HashMaps (graphs)
    let mut graphs: [Graph; 11] = Default::default();

    // Iterate through the nodes
    for node in nodes {
        // Determine which graph the node belongs to
        let index = node.stub_name_num as usize;
        println!("stub_num_num: {}", node.stub_name_num);

        

        if index < 11 {
            // Use a unique identifier for the node, e.g., "year-age"
            let node_id = format!("{}-{}", node.year, node.age);

            // Insert the node into the appropriate graph
            graphs[index].insert(node_id, node);
        }
    }

    graphs
}
