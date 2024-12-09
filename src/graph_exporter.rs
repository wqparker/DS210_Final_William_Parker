use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};

/// Exports the graph edges to a CSV file.
/// 
/// # Parameters:
/// - `edge_map`: A reference to the edge map (HashMap where key is the node ID, and value is a Vec of (target node ID, weight)).
/// - `output_file`: Path to the CSV file to be created.
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
