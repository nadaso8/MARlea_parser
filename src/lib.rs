/// Author: Marceline Sorensen 
/// Email: nadaso8th@gmail.com 
/// Date: 12/24/2023
/// 
/// # Description
/// This is the parser module for the Marlea D-CRN simulator. 
/// Its purpose it to take a variety of plaintext source files such as .csv or .rs and compile a reaction network, 
/// which may be simulated by the [MARlea_engine](https://github.com/nadaso8/MARlea_engine) module.

use std::path::{Path, self};

use pest::Parser;
use pest_derive::Parser;

use marlea_engine::trial::reaction_network::ReactionNetwork;

// derive parsers 
#[derive(Parser)]
#[grammar = "grammars/csv.pest"]
struct CSVparser;


enum marlea_parser_error {
    unknown(String)
}

// object containing any settings needed or relevant to the marlea parser 
struct marlea_parser;

impl marlea_parser {
    pub fn new() -> Self{
        Self
    }

    /// Parses a reaction network and solution from a variety of file types 
    pub fn parse(path: &Path) -> Result<ReactionNetwork,marlea_parser_error> {
        let result = Result::Err(marlea_parser_error::unknown("failed to parse file".to_string()));

        todo!("make the parser");
        
        return result;
    }
}