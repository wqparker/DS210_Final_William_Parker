mod node;
mod data_loader;

use data_loader::load_data_from_csv;

fn main() {
    // Define the file path to your CSV file
    let file_path = "data.csv";

    // Load the data
    let data = load_data_from_csv(file_path);

    // Check if the data was loaded successfully
    if data.is_empty() {
        println!("No data was loaded. Please check the file path or the file format.");
    } else {
        println!("Data loaded successfully! Total rows: {}", data.len());

        // Print the first few entries (e.g., the first 3 rows)
        for (i, entry) in data.iter().take(3).enumerate() {
            println!("Row {}: {:?}", i + 1, entry);
        }
    }
}
