use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};

// reads in a graph represented as a hashmap of nodes and their connected nodes with weighted edges, and string for what to call output file
// outputs a csv file in the format of the conencted edges, ie "node a, node b, weight"
pub fn export_graph_to_csv(
    edge_map: &HashMap<String, Vec<(String, f64)>>,
    output_file: &str,
) {
    let file = File::create(output_file).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    // Write CSV headers
    writeln!(writer, "Source,Target,Weight").expect("Unable to write to file");

    // Write edges
    for (node_id, edges) in edge_map {
        for (target_node_id, weight) in edges {
            writeln!(writer, "{},{},{:.2}", node_id, target_node_id, weight)
                .expect("Unable to write to file");
        }
    }

    println!("Graph exported to {}", output_file);
}
