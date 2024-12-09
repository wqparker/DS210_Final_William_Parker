use crate::node::Node;

pub fn calculate_average_estimate(data: &[Node]) -> f64 {
    let sum: f64 = data.iter().map(|node| node.estimate).sum();
    sum / data.len() as f64
}

pub fn group_by_year(data: &[Node]) -> Vec<(String, Vec<&Node>)> {
    let mut groups = std::collections::HashMap::new();

    for node in data {
        groups.entry(node.year.clone())
            .or_insert_with(Vec::new)
            .push(node);
    }

    groups.into_iter().collect()
}
