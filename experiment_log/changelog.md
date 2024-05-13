## Log

### Date 
9th February 2024

### Changes
- Basic setup with graph structure and the nodes. Only allowing SNOMED codelists which are hard coded in. Able to produce 20 records with 5 patietns at present with fake NHS numbers. 
- Add in episodes of illness so there is a chance (1 in 3) that each code would present with multiple events from within the same codelist.

### Things to think about:
- Embedding the codelists if they come with a specific structure using the `rust_embed` [macro](https://crates.io/crates/rust-embed).
- How to map SNOMED to ICD10 if we want to do this. It might be easier to just make people add their own ICD10 codelist. 
