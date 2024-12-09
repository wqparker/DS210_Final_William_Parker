use crate::node::Node;
use csv::Reader;
use std::error::Error;

pub fn load_data_from_csv(file_path: &str) -> Vec<Node> {
    let mut reader = match Reader::from_path(file_path) {
        Ok(r) => r,
        Err(e) => {
            println!("Error opening file: {}", e);
            return Vec::new();
        }
    };

    let mut data = Vec::new();

    for result in reader.records() {
        match result {
            Ok(record) => {
                let node = Node {
                    unit: record[1].to_string(),
                    unit_num: record[2].parse().unwrap_or_default(),
                    stub_name: record[3].to_string(),
                    stub_name_num: record[4].parse().unwrap_or_default(),
                    stub_label: record[5].to_string(),
                    stub_label_num: record[6].parse().unwrap_or_default(),
                    year: record[7].to_string(),
                    year_num: record[8].parse().unwrap_or_default(),
                    age: record[9].to_string(),
                    age_num: record[10].parse().unwrap_or_default(),
                    estimate: record[11].parse().unwrap_or_default(),
                };
                data.push(node);
            }
            Err(e) => {
                println!("Error reading a row: {}", e);
            }
        }
    }

    data
}
