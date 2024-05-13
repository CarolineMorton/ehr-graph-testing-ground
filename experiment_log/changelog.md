## Log
### Date 
9th February 2024

### Changes
- Basic setup with graph structure and the nodes. Only allowing SNOMED codelists which are hard coded in. Able to produce 20 records with 5 patietns at present with fake NHS numbers. 
- Add in episodes of illness so there is a chance (1 in 3) that each code would present with multiple events from within the same codelist.

### Things to think about:
- Embedding the codelists if they come with a specific structure using the `rust_embed` [macro](https://crates.io/crates/rust-embed).
- How to map SNOMED to ICD10 if we want to do this. It might be easier to just make people add their own ICD10 codelist. 

---
### Date 
10th March 2024

### Changes
- None 

### Things to think about:
- I think it will be easier to create a population of Patient structs as the output of the tool. We can then create adapters (using [adapter pattern](https://refactoring.guru/design-patterns/adapter/) to create a CPRD adapter to output the CPRD flat files, and any other types of files that we might need. I have used the adapter pattern quite a lot in the past and found it to be helpful.

---
### Date 
29th April 2024

### Changes
- None 

### Things to think about:
- Run as a CLI in the initial output but ideally would be a website. Need to think about what the UI would look like. 
