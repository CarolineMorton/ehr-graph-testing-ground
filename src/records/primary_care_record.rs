//! This file contains the PrimaryCareRecord struct and its methods.

// External crates
use serde::{Deserialize, Serialize};
use rand::Rng;
use rand_distr::{Distribution, Normal};



// Internal modules
use crate::codelists::codelists::CodeList;


fn generate_year(start_year: i32, mean_year: f64, std_dev: f64) -> i32 {
    let normal = Normal::new(mean_year, std_dev).unwrap();

    let mut rng = rand::thread_rng();
    let year = loop {
        let sample = normal.sample(&mut rng).round() as i32;
        if sample >= start_year && sample <= 2024 {
            break sample;
        }
    };

    year
}


/// A primary care record is a record of a patient's visit to their primary care physician.
///
/// # Arguments
/// * `patient_id` - The patient's unique identifier.
/// * `code` - The code for the condition
/// * `term` - The term used to describe the condition
/// * `date` - The date of the visit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimaryCareRecord {
    pub patient_id: String,
    pub code: i32,
    pub term: String,
    pub date: String,
}

impl PrimaryCareRecord {
    /// Generate a new primary care record.
    ///
    /// # Arguments
    /// * `patient_id` - The patient's unique identifier.
    /// * `code` - The code for the condition
    /// * `term` - The term used to describe the condition
    /// * `date` - The date of the visit
    ///
    /// # Returns
    /// * A new primary care record.
    pub fn new(patient_id: String, code: i32, term: String, date: String) -> PrimaryCareRecord {
        PrimaryCareRecord {
            patient_id,
            code,
            term,
            date,
        }
    }
}

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
        let mut mean_year = year_of_birth + codelist.mean_age;

        if mean_year >= 2024 {
            mean_year = 2024
        }

        let year = generate_year(year_of_adult, mean_year.into(), 10.0);
        let month = rand::thread_rng().gen_range(1..12);
        let day = rand::thread_rng().gen_range(1..28);

        // Day and Months need to be padded with a zero if they are less than 10
        let month = format_date(month);
        let day = format_date(day);

        // Format the date
        let date = format!("{}-{}-{}", year, month, day);

        PrimaryCareRecord {
            patient_id,
            code: selected_code.code,
            term: selected_code.term.clone(),
            date: date,
        }
    }
}


/// Format a date digit to a string, padding with a zero if necessary.
///
/// # Arguments
/// * `date_digit` - A date digit.
///
/// # Returns
/// * A string representation of the date digit.
fn format_date(date_digit: i32) -> String {
    if date_digit < 10 {
        return format!("0{}", date_digit)
    } else {
        return date_digit.to_string()
    };
}