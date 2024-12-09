mod node;
mod data_loader;

use data_loader::load_data_from_csv;

fn main() {
    let file_path = "data.csv";

    // Load all nodes from the CSV
    let nodes = load_data_from_csv(file_path);

    if nodes.is_empty() {
        println!("No data was loaded. Please check the file path or format.");
        return;
    }

    println!("Total nodes loaded: {}", nodes.len());

    // Separate nodes into two groups based on unit_num
    let (group1, group2): (Vec<_>, Vec<_>) = nodes.into_iter()
        .partition(|node| node.unit_num == 1);

    // Print the counts of each group
    println!("Group 1 (unit_num == 1): {} nodes", group1.len());
    println!("Group 2 (unit_num == 2): {} nodes", group2.len());

    // (Optional) Print some nodes from each group for verification
    println!("Sample from Group 1:");
    for (i, node) in group1.iter().take(5).enumerate() {
        println!("Node {}: {:?}", i + 1, node);
    }

    println!("Sample from Group 2:");
    for (i, node) in group2.iter().take(5).enumerate() {
        println!("Node {}: {:?}", i + 1, node);
    }
}
