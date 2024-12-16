mod node;
mod data_loader;
mod analysis; // Import the analysis module for Louvain Algorithm
mod graph_exporter;
mod graph;

use crate::node::Node; // Import the Node struct
use crate::data_loader::load_data_from_csv;
use crate::graph_exporter::export_graph_to_csv;
use crate::graph::{split_by_unit_num, subdivide_by_stub_name_num, build_graph};
use crate::analysis::{run_label_propagation};

use std::collections::HashMap;
// main function to work with data
fn main() {
    // file path
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

    // Work with only the 6th group (index 5)
    if let Some(subgroup) = subgroups.get(5) {
        if !subgroup.is_empty() {
            let graph = build_graph(subgroup.clone()); // Pass Vec<Node> directly to build_graph
            println!("Graph 6 has {} nodes.", graph.len());

            // Run LPA alg on data
            //let new_graph = run_label_propagation(&graph);

            export_graph_to_csv(&graph, "graph_6.csv");

        } else {
            println!("Graph 6 does not exist.");
        }
    } else {
        println!("Subgroup 6 (index 5) is out of bounds.");
    }     
}
