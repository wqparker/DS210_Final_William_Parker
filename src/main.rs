mod node;
mod data_loader;
mod graph;
mod graph_exporter;

use crate::node::Node; // Import the Node struct
use crate::data_loader::load_data_from_csv;
use crate::graph::{split_by_unit_num, subdivide_by_stub_name_num, build_graph};
use crate::graph_exporter::{export_graph_to_csv};

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

    // Print the number of nodes in graph 5 (index 5 of the subgroups)
    if let Some(subgroup) = subgroups.get(5) {
        let graph_5 = build_graph(subgroup.clone()); // Clone subgroup to pass into the function
        println!("Graph 5 has {} nodes", graph_5.len());
        // Export Graph 5 to a CSV file
        export_graph_to_csv(&graph_5, "graph_5.csv");

        // Print an example node from graph 5
        if let Some((example_node_id, example_edges)) = graph_5.iter().next() {
            //println!(
            //    "Example node ID: {}\nConnected nodes and weights: {:?}",
            //    example_node_id, example_edges
            //);
        } else {
            println!("Graph 5 is empty.");
        }
    } else {
        println!("Graph 5 does not exist.");
    }
    
    
}
