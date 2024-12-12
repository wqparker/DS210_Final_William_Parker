mod node;
mod data_loader;
mod edge_assigner;
mod analysis; // Import the analysis module for Louvain Algorithm
mod graph_exporter;
mod graph;

use crate::node::Node; // Import the Node struct
use crate::data_loader::load_data_from_csv;
use crate::graph_exporter::export_graph_to_csv;
use crate::graph::{split_by_unit_num, subdivide_by_stub_name_num, build_graph};
use crate::analysis::louvain; // Import the Louvain function
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
    let (group_1, group_2) = split_by_unit_num(nodes);

    println!(
        "Group 1 (unit_num == 1): {} nodes\nGroup 2 (unit_num != 1): {} nodes",
        group_1.len(),
        group_2.len()
    );


    // Subdivide Group 2 into 11 subvectors based on `stub_name_num`
    let subgroups = subdivide_by_stub_name_num(group_2);

    for (index, subgroup) in subgroups.into_iter().enumerate() {
        // Check if the subgroup has elements
        if !subgroup.is_empty() {
            // Use the Vec<Node> directly
            let graph = build_graph(subgroup); // Pass Vec<Node> directly to build_graph
            println!("Graph {} has {} nodes.", index + 1, graph.len());
    
            // Export the graph to CSV
            let file_name = format!("graph_{}.csv", index + 1);
            export_graph_to_csv(&graph, &file_name);
            println!("Graph {} exported to {}.", index + 1, file_name);
        } else {
            println!("Graph {} does not exist.", index + 1);
        }
    }     
}
