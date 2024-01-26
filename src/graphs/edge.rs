//! The file contains the Edge struct and its methods.

// External crates
use serde::{Deserialize, Serialize};

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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub from: Node,
    pub to: Node,
    pub weight: i32,
}
