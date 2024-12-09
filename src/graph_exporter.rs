use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};
use crate::node::Node;

pub fn export_graph_to_csv(graph: &HashMap<String, Node>, output_file: &str) {
    let file = File::create(output_file).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    // Write CSV headers
    writeln!(writer, "Source,Target,Weight").expect("Unable to write to file");

    // Write edges
    for (node_id, node) in graph {
        for edge in &node.edges {
            writeln!(
                writer,
                "{},{},{:.2}",
                node_id, edge.target, edge.weight
            )
            .expect("Unable to write to file");
        }
    }

    println!("Graph exported to {}", output_file);
}
