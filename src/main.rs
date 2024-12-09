mod node;
mod data_loader;
mod edge_assigner;

use crate::node::Node; // Import the Node struct
use crate::data_loader::load_data_from_csv;
use crate::edge_assigner::assign_edges_in_hashmaps;
use std::collections::HashMap;

fn main() {
    let file_path = "data.csv";

    // Load nodes from the CSV
    let nodes = load_data_from_csv(file_path);
    println!("Total nodes loaded: {}", nodes.len());

    if nodes.is_empty() {
        println!("No data was loaded. Please check the file path or format.");
        return;
    }

    // Initialize HashMaps for each stub_name_num
    let mut maps: [HashMap<String, Node>; 12] = Default::default();

    // Distribute nodes into HashMaps
    for node in nodes {
        let index = node.stub_name_num as usize;
        if index < maps.len() {
            let node_id = format!(
                "{}-{}-{}",
                node.stub_label_num, node.year_num, node.age_num
            );
            maps[index].insert(node_id, node);
        }
    }

    // Assign edges within each HashMap
    assign_edges_in_hashmaps(&mut maps);

    // Print sample output for verification
    for (i, map) in maps.iter().enumerate() {
        println!("HashMap {} has {} nodes", i, map.len());
        if let Some((_, node)) = map.iter().next() {
            println!("Sample Node: {:?}", node);
        }
    }
}
