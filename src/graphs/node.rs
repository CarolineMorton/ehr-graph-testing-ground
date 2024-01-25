//! This file contains the Node struct and its methods.

// Internal modules
use crate::codelists::codelists::CodeList;


/// A node is a node in a DAG. The node represents a variable of 
/// interest in the dataset. The node has at least one codelist 
/// associated with it.
///
/// # Arguments
/// * `codelists` - A vector of codelists
/// * `name` - The name of the node
pub struct Node {
    pub codelists: Vec<CodeList>,
    pub name: String,
}