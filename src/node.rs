#[derive(Debug, Clone, PartialEq)]
// define our node struct to hold the data we want, formatted to our data
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_initialization() {
        // Create a Node instance using the constructor
        let node = Node::new(
            "Deaths per 100,000",                          // unit
            2,                                         // unit_num
            "Sex and race",                           // stub_name
            4,                                    // stub_name_num
            "Male: American Indian or Alaska Native",// stub_label
            4.13,                                // stub_label_num
            "1988",                                        // year
            12,                                        // year_num
            "All ages",                                     // age
            0,                                          // age_num
            20.2,                                      // estimate
        );

        // Define the expected Node
        let expected = Node {
            unit: "Deaths per 100,000".to_string(),
            unit_num: 2,
            stub_name: "Sex and race".to_string(),
            stub_name_num: 4,
            stub_label: "Male: American Indian or Alaska Native".to_string(),
            stub_label_num: 4.13,
            year: "1988".to_string(),
            year_num: 12,
            age: "All ages".to_string(),
            age_num: 0,
            estimate: 20.2,
        };

        // Check equality
        assert_eq!(node, expected);
    }

    #[test]
    fn test_node_debug_format() {
        // Create a Node instance
        let node = Node::new(
            "Deaths per 100,000", 2, "Sex and race", 4,
            "Male: American Indian or Alaska Native", 4.13,
            "1988", 12, "All ages", 0, 20.2,
        );

        // Verify the Debug output (formatted string)
        let debug_output = format!("{:?}", node);
        let expected_output = "Node { unit: \"Deaths per 100,000\", unit_num: 2, stub_name: \"Sex and race\", stub_name_num: 4, stub_label: \"Male: American Indian or Alaska Native\", stub_label_num: 4.13, year: \"1988\", year_num: 12, age: \"All ages\", age_num: 0, estimate: 20.2 }";

        assert_eq!(debug_output, expected_output);
    }
}
