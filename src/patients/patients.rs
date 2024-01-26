//! This file contains the Patient struct and its methods.

// External crates
use serde::{Deserialize, Serialize};

// Internal modules
use crate::records::primary_care_record::PrimaryCareRecord;


/// A patient is a person who has the potential to be in the population.
///
/// # Arguments
/// * `patient_id` - The patient's unique identifier.
/// * `year_of_birth` - The patient's year of birth.
/// * `records` - The patient's records.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Patient {
    pub patient_id: String,
    pub year_of_birth: i32,
    pub gender: String,
    pub records: Vec<PrimaryCareRecord>,
}
