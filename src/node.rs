#[derive(Debug)]
pub struct Node {
    pub unit: String,          // Deaths per 100,000 unit
    pub unit_num: i32,         // 1 or 2 / adjusted or crude
    pub stub_name: String,     // demographic/s
    pub stub_name_num: i32,    // 1 - 11
    pub stub_label: String,    // category in demographic
    pub stub_label_num: f64,   // 1.0 - 11.0
    pub year: String,          // year
    pub year_num: i32,         // 1 - 50
    pub age: String,           // age group
    pub age_num: i32,          // 1 - 5
    pub estimate: f64,         // estimate rate, 0.0 - __.0
    pub edges: Vec<Edge>,      // List of edges connecting this node to others
}

impl Node {
    pub fn new(
        unit: &str, unit_num: i32, stub_name: &str, stub_name_num: i32,
        stub_label: &str, stub_label_num: f64, year: &str, year_num: i32,
        age: &str, age_num: i32, estimate: f64,
    ) -> Self {
        Self {
            unit: unit.to_string(),
            unit_num,
            stub_name: stub_name.to_string(),
            stub_name_num,
            stub_label: stub_label.to_string(),
            stub_label_num,
            year: year.to_string(),
            year_num,
            age: age.to_string(),
            age_num,
            estimate,
            edges: Vec::new(), // Initialize edges as an empty vector
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    pub target: String, // Node ID of the connected node
    pub weight: f64,    // Weight of the connection
}
