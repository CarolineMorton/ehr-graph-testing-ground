//! This file contains the Code

// External crates
use serde::{Deserialize, Serialize};


/// A code is a code (like ICD10 or SNOMED) and terms that are used to 
/// describe a condition, disease state or finding.
///
/// # Arguments
/// * `code` - The code for the condition
/// * `term` - The term used to describe the condition
/// * `frequency` - The frequency of the code in the dataset
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Code {
    pub code: i32,
    pub term: String,
    pub frequency: i32,
}