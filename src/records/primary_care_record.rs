//! This file contains the PrimaryCareRecord struct and its methods.

// External crates
use serde::{Deserialize, Serialize};
use rand::Rng;

// Internal modules
use crate::codelists::codelists::CodeList;



/// A primary care record is a record of a patient's visit to their primary care physician.
///
/// # Arguments
/// * `patient_id` - The patient's unique identifier.
/// * `code` - The code for the condition
/// * `term` - The term used to describe the condition
/// * `date` - The date of the visit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryCareRecord {
    pub patient_id: String,
    pub code: i32,
    pub term: String,
    pub date: String,
}

// impl PrimaryCareRecord {
//     /// Generate a new primary care record.
//     ///
//     /// # Arguments
//     /// * `patient_id` - The patient's unique identifier.
//     /// * `code` - The code for the condition
//     /// * `term` - The term used to describe the condition
//     /// * `date` - The date of the visit
//     ///
//     /// # Returns
//     /// * A new primary care record.
//     pub fn new(patient_id: String, code: i32, term: String, date: String) -> PrimaryCareRecord {
//         PrimaryCareRecord {
//             patient_id,
//             code,
//             term,
//             date,
//         }
//     }
// }

/// A trait for generating a primary care record.
pub trait RowGenerator {
    fn generate(patient_id: String, year_of_birth: i32, codelist: CodeList) -> PrimaryCareRecord;
}

/// Implementation of the RowGenerator trait for the PrimaryCareRecord struct.
impl RowGenerator for PrimaryCareRecord {
    /// Generate a primary care record.
    ///
    /// # Arguments
    /// * `patient_id` - The patient's unique identifier.
    /// * `codelist` - A codelist.
    ///
    /// # Returns
    /// * A new primary care record.
    fn generate(patient_id: String, year_of_birth: i32, codelist: CodeList) -> PrimaryCareRecord {
        let num_rows = rand::thread_rng().gen_range(1..5);

        let codes = codelist.codes;
        let total_frequency: i32 = codes.iter().map(|c| c.frequency).sum();

        if total_frequency == 0 {
            panic!("Total frequency is zero, cannot generate a code");
        }

        let mut rng = rand::thread_rng();
        let mut random_number = rng.gen_range(0..total_frequency);

        let mut selected_code = &codes[0]; // Default to first code, replace in loop
        for code in &codes {
            random_number -= code.frequency;
            if random_number <= 0 {
                selected_code = code;
                break;
            }
        }

        // Pick a random date between 18 years old and 2022
        let year_of_adult = year_of_birth + 18;
        let year = rand::thread_rng().gen_range(year_of_adult..2022);
        let month = rand::thread_rng().gen_range(1..12);
        let day = rand::thread_rng().gen_range(1..28);

        // Day and Months need to be padded with a zero if they are less than 10
        let day = if day < 10 {
            format!("0{}", day)
        } else {
            day.to_string()
        };

        let month = if month < 10 {
            format!("0{}", month)
        } else {
            month.to_string()
        };
        let date = format!("{}-{}-{}", year, month, day);

        PrimaryCareRecord {
            patient_id,
            code: selected_code.code,
            term: selected_code.term.clone(),
            date: date,
        }
    }
}
