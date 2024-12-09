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
                    unit: record[0].to_string(),
                    unit_num: record[1].parse().unwrap_or_default(),
                    stub_name: record[2].to_string(),
                    stub_name_num: record[3].parse().unwrap_or_default(),
                    stub_label: record[4].to_string(),
                    stub_label_num: record[5].parse().unwrap_or_default(),
                    year: record[6].to_string(),
                    year_num: record[7].parse().unwrap_or_default(),
                    age: record[8].to_string(),
                    age_num: record[9].parse().unwrap_or_default(),
                    estimate: record[10].parse().unwrap_or_default(),
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
