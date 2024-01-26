//! This file defines the Graph struct and its methods.

// External crates
use serde::{Deserialize, Serialize};

// Internal modules
use super::node::Node;
use super::edge::Edge;



/// A graph is a directed acyclic graph (DAG) that represents the
/// relationships between variables of interest in the dataset.
///
/// # Arguments
/// * `nodes` - A vector of nodes
/// * `edges` - A vector of edges
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Option<Vec<Node>>,
    pub edges: Option<Vec<Edge>>,
}
