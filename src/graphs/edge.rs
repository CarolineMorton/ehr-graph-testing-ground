//! The file contains the Edge struct and its methods.

// Internal modules
use super::node::Node;

/// An edge is a directed edge between two nodes in a DAG. The edge
/// represents a relationship between two variables of interest in the
/// dataset.
///
/// # Arguments
/// * `from` - The node from which the edge originates
/// * `to` - The node to which the edge points
/// * `weight` - The weight of the edge
pub struct Edge {
    pub from: Node,
    pub to: Node,
    pub weight: i32,
}
