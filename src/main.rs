/// This is the main file for the project. It contains the main function. 

// External crates
mod patients;
mod records;
mod codelists;
mod graphs;

// Internal modules
use crate::patients::population::{Population, PopulationGenerator, RecordGenerator};
use crate::codelists::codelists::CodeList;
use crate::codelists::code::Code;
use crate::graphs::graph::Graph;
use crate::graphs::node::Node;
use crate::patients::population::Instructions;


fn main() {
    let pneumonia_snomed = CodeList {
        name: "Pneumonia".to_string(),
        codes: vec![
            Code {
                code: 233604007,
                term: "Pneumonia".to_string(),
                frequency: 70,
            },
            Code {
                code: 68409003,
                term: "Organised Pneumonia".to_string(),
                frequency: 5,
            },
            Code {
                code: 75570004,
                term: "Viral Pneumonia".to_string(),
                frequency: 15,
            },
            Code {
                code: 53084003,
                term: "Bacterial Pneumonia".to_string(),
                frequency: 10,
            },
        ],
        mean_age: 52,
        coding_system: "SNOMED".to_string(),
        setting: "primary care".to_string(),
    };

    let uti_snomed = CodeList {
        name: "UTI".to_string(),
        codes: vec![
            Code {
                code: 12345678,
                term: "UTI".to_string(),
                frequency: 80,
            },
            Code {
                code: 123123212,
                term: "Serious UTI".to_string(),
                frequency: 20,
            },
        ],
        mean_age: 70,
        coding_system: "SNOMED".to_string(),
        setting: "primary care".to_string(),
    };



    // Create a graph
    let mut graph = Graph {
        nodes: Some(Vec::new()),
        edges: None,
    };

    // Generate the nodes
    let first_node = Node {
        codelists: vec![pneumonia_snomed.clone(), uti_snomed.clone()],
        name: "Respiratory Infection".to_string(),
    };

    if let Some(ref mut nodes) = graph.nodes {
        nodes.push(first_node);
    } else {
        graph.nodes = Some(vec![first_node]);
    }

    // Create the instructions
    let instructions = Instructions {
        population_size: 1,
        graph: graph.clone(),
    };

    // Define a patient population
    let mut population = Population {
        name: "Test Study Cohort".to_string(),
        description: "A cohort of patients".to_string(),
        instructions: instructions.clone(),
        patients: None,
        records: None,
    };

    // Generate the patients
    population.generate_population();
    println!("Patients Generated");
    for patient in population.patients.clone().unwrap() {
        println!("Patient ID: {}  - Year of Birth: {} - Gender: {}", patient.patient_id, patient.year_of_birth, patient.gender);
    } 

    // Generate the rows of primary care data by iterating over the nodes in the graph
    for node in instructions.graph.nodes.clone().unwrap() {
        for codelist in node.codelists {
            for _patient in population.patients.clone().unwrap() {
                    // Generate the records
                    population.generate_records(codelist.clone());
                    }
                }
            }
        
    // population.generate_records(pneumonia_snomed.clone());
    println!("\nRecords Generated");

    // Print the rows of primary care data
    population.print_rows();
}
