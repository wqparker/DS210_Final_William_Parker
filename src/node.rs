#[derive(Debug)]
pub struct Node {
    pub unit: String,
    pub unit_num: i32,
    pub stub_name: String,
    pub stub_name_num: i32,
    pub stub_label: String,
    pub stub_label_num: f64,
    pub year: String,
    pub year_num: i32,
    pub age: String,
    pub age_num: i32,
    pub estimate: f64,
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
        }
    }
}
    