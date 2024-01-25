/// This is the main file for the project. It contains the main function. 

// External crates
mod patients;
mod records;
mod codelists;
// mod graphs;

// Internal modules
use crate::patients::population::{Population, PopulationGenerator, RecordGenerator};
use crate::codelists::codelists::CodeList;
use crate::codelists::code::Code;


fn main() {
    let pneumonia_snomed = CodeList {
        codes: vec![
            Code {
                code: 233604007,
                term: "Pneumonia".to_string(),
                frequency: 80,
            },
            Code {
                code: 68409003,
                term: "Organised Pneumonia".to_string(),
                frequency: 20,
            },
        ],
        coding_system: "SNOMED".to_string(),
        setting: "primary care".to_string(),
    };

    // Define a patient population
    let mut population = Population {
        name: "Test Study Cohort".to_string(),
        description: "A cohort of patients".to_string(),
        instructions: 5,
        patients: None,
        records: None,
    };

    // Generate the patients
    population.generate_population();
    println!("Patients Generated");
    for patient in population.patients.clone().unwrap() {
        println!("Patient ID {}  - Year of Birth: {}", patient.patient_id, patient.year_of_birth);
    } 

    // Generate the rows of primary care data
    population.generate_records(pneumonia_snomed.clone());
    println!("\nRecords Generated");

    // Print the rows of primary care data
    population.print_rows();
}
