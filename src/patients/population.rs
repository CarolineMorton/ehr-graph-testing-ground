//! This file contains the Population struct and its methods.

// External crates
use serde::{Deserialize, Serialize};
use rand::Rng;
use rand::prelude::SliceRandom;
use chrono::{NaiveDate, Duration};

// Internal modules
use crate::patients::patients::Patient;
use crate::records::primary_care_record::{PrimaryCareRecord, RowGenerator};
use crate::codelists::codelists::CodeList;
use crate::graphs::graph::Graph;

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
    pub instructions: Instructions,
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
        for _i in 0..self.instructions.population_size {

            let mut rng = rand::thread_rng();
            let options = ["Male", "Female"];
            let gender = options.choose(&mut rng).unwrap();

            let patient = Patient {
                patient_id: rand::thread_rng().gen_range(100000000..999999999).to_string(),
                gender: gender.to_string(),
                year_of_birth: rand::thread_rng().gen_range(1920..1980),
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
        
        // Loop through the patients (.patients)
        let patients = self.patients.clone().unwrap();
        for patient in patients {
            // chose a random number of rows to generate
            let num_rows = rand::thread_rng().gen_range(0..3);

            // Loop through the rows
            for _i in 0..num_rows {
                // Generate a row
                let row = PrimaryCareRecord::generate(patient.patient_id.clone(), patient.year_of_birth.clone(), codelist.clone());
                // Add the row to the rows vector as mutable
                if self.records == None {
                    let mut new_records = Vec::new();
                    new_records.push(row.clone());
                    self.records = Some(new_records);
                } else {
                    self.records.as_mut().unwrap().push(row.clone());
                }

                // Create episode of care for the row by randomly selecting a random number between 0 and 3
                // and creating new rows for this number with a date that is within 30 days of the original row
                let num_rows = rand::thread_rng().gen_range(0..3);

                for _i in 0..num_rows {
                    let mut rng = rand::thread_rng();
                    let days = rng.gen_range(0..30);
                    let date = row.date.clone();
                    let date = date.parse::<NaiveDate>().unwrap();
                    let new_date = date + Duration::days(days);
                    let new_date = new_date.format("%Y-%m-%d").to_string();
                    
                    // TODO: This is a bit of a hack, but it works for now. Need to refactor this so that we call the generate method
                    // on the PrimaryCareRecord struct, rather than creating a new row. This will allow us to pick a code from the codelist
                    // that is related to the original code, rather than cloning the original code.
                    let row = PrimaryCareRecord::new(row.patient_id.clone(), row.code.clone(), row.term.clone(), new_date);
                    self.records.as_mut().unwrap().push(row.clone());
                }

            }
        }
    }

    /// Print the rows of the population.
    fn print_rows(&self) {
        let rows = self.records.clone().unwrap();
        for row in rows {
            println!("{:?}", row);
        }
    }
}


/// A struct for holding the instructions for generating a population.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Instructions {
    pub population_size: i32,
    pub graph: Graph
}

