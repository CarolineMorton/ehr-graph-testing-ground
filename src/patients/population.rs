//! This file contains the Population struct and its methods.

// External crates
use serde::{Deserialize, Serialize};
use rand::Rng;

// Internal modules
use crate::patients::patients::Patient;
use crate::records::primary_care_record::{PrimaryCareRecord, RowGenerator};
use crate::codelists::codelists::CodeList;


/// A population is a group of patients and their records.
///
/// # Arguments
/// * `name` - The name of the population.
/// * `description` -The description of the population.
/// * `instructions` - The number of patients in the population.
/// * `patients` - The patients in the population.
/// * `records` - The records in the population.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Population {
    pub name: String,
    pub description: String,
    pub instructions: i32,
    pub patients: Option<Vec<Patient>>,
    pub records: Option<Vec<PrimaryCareRecord>>,
}


/// A trait for generating a population.
pub trait PopulationGenerator {
    fn generate_population(&mut self);
}

/// Implementation of the PopulationGenerator trait for the Population struct.
impl PopulationGenerator for Population {

    /// Generate a population, and assign it to the patients field of the Population struct.
    fn generate_population(&mut self) {
        let mut patients: Vec<Patient> = Vec::new();
        for _i in 0..self.instructions {
            let patient = Patient {
                patient_id: rand::thread_rng().gen_range(100000000..999999999).to_string(),
                year_of_birth: rand::thread_rng().gen_range(1930..2003),
                records: Vec::new(),
            };
            patients.push(patient);
        }
        self.patients = Some(patients.clone());
    }
}

/// A trait for generating records.
pub trait RecordGenerator {
    fn generate_records(&mut self, codelist: CodeList);
    fn print_rows(&self);
}

impl RecordGenerator for Population {
    /// Generate records for the population.
    ///
    /// # Arguments
    /// * `codelist` - The codelist to use for generating the records.
    fn generate_records(&mut self, codelist: CodeList) {
        let mut rows: Vec<PrimaryCareRecord> = Vec::new();
        
        // Loop through the patients (.patients)
        let patients = self.patients.clone().unwrap();
        for patient in patients {
            // chose a random number of rows to generate
            let num_rows = rand::thread_rng().gen_range(1..5);

            // Loop through the rows
            for _i in 0..num_rows {
                // Generate a row
                let row = PrimaryCareRecord::generate(patient.patient_id.clone(), patient.year_of_birth.clone(), codelist.clone());
                // Add the row to the rows vector
                rows.push(row);
            }
        }
        self.records = Some(rows.clone());
    }

    /// Print the rows of the population.
    fn print_rows(&self) {
        let rows = self.records.clone().unwrap();
        for row in rows {
            println!("{:?}", row);
        }
    }
}
