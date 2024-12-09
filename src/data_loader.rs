use crate::node::Node;
use csv::Reader;

pub fn load_data_from_csv(file_path: &str) -> Vec<Node> {
    let mut reader = match Reader::from_path(file_path) {
        Ok(r) => r,
        Err(e) => {
            println!("Error opening file: {}", e);
            return Vec::new();
        }
    };

    let mut nodes = Vec::new();

    for (i, result) in reader.records().enumerate() {
        match result {
            Ok(record) => {
                let node = Node::new(
                    &record[1],                    // unit
                    record[2].parse().unwrap_or_default(), // unit_num
                    &record[3],                    // stub_name
                    record[4].parse().unwrap_or_default(), // stub_name_num
                    &record[5],                    // stub_label
                    record[6].parse().unwrap_or_default(), // stub_label_num
                    &record[7],                    // year
                    record[8].parse().unwrap_or_default(), // year_num
                    &record[9],                    // age
                    record[10].parse().unwrap_or_default(), // age_num
                    record[11].parse().unwrap_or_default(), // estimate
                );
                nodes.push(node);
            }
            Err(e) => {
                println!("Error reading row {}: {}", i + 1, e);
            }
        }
    }

    nodes
}
