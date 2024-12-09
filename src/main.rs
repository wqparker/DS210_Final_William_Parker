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

    // Split nodes into two groups based on `unit_num`
    let mut group_1: Vec<Node> = Vec::new();
    let mut group_2: Vec<Node> = Vec::new();

    for node in nodes {
        if node.unit_num == 1 {
            group_1.push(node);
        } else {
            group_2.push(node);
        }
    }

    println!(
        "Group 1 (unit_num == 1): {} nodes\nGroup 2 (unit_num != 1): {} nodes",
        group_1.len(),
        group_2.len()
    );

    // Initialize HashMaps for each stub_name_num in Group 2
    let mut maps: [HashMap<String, Node>; 12] = Default::default();

    // Distribute Group 2 nodes into HashMaps
    for node in group_2 {
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
    }

    // Analyze Graph 9
    println!("\nNodes and Edges in Graph 9:");
    if let Some(graph_9) = maps.get(9) {
        for (node_id, node) in graph_9.iter() {
            println!("Node ID: {}", node_id);
            for edge in &node.edges {
                println!("  Connected to: {}, Weight: {:.2}", edge.target, edge.weight);
            }
        }

        // Total edges in Graph 9
        let total_edges: usize = graph_9.values().map(|node| node.edges.len()).sum();
        println!("Total edges in Graph 9: {}", total_edges);
    } else {
        println!("Graph 9 is empty.");
    }

    // Analyze edge density for all graphs
    println!("\nGraph Edge Density:");
    for (i, map) in maps.iter().enumerate() {
        let node_count = map.len();
        let edge_count: usize = map.values().map(|node| node.edges.len()).sum();
        let avg_connections = if node_count > 0 {
            edge_count as f64 / node_count as f64
        } else {
            0.0
        };
        println!(
            "Graph {}: {} nodes, {} edges, {:.2} avg connections per node",
            i, node_count, edge_count, avg_connections
        );
    }
}
