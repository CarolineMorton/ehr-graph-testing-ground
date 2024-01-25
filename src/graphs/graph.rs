//! This file defines the Graph struct and its methods.




/// A graph is a directed acyclic graph (DAG) that represents the
/// relationships between variables of interest in the dataset.
///
/// # Arguments
/// * `nodes` - A vector of nodes
/// * `edges` - A vector of edges
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
