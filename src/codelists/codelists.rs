//! This contains the Codelist struct and its methods.

// External crates
use serde::{Deserialize, Serialize};

// Internal modules
use crate::codelists::code::Code;


/// A codelist is a list of codes that are used to describe a
/// condition, disease state or finding.
///
/// # Arguments
/// * `codes` - A vector of codes
/// * `coding_system` - The coding system used for the codelist e.g. ICD10, SNOMED
/// * `setting` - The setting in which the codelist is used e.g. primary care, secondary care
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeList {
    pub name: String,
    pub mean_age: i32,
    pub codes: Vec<Code>,
    pub coding_system: String,
    pub setting: String,
}