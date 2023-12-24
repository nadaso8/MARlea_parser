/// Author: Marceline Sorensen 
/// Email: nadaso8th@gmail.com 
/// Date: 12/24/2023
/// 
/// # Description
/// This is the parser module for the Marlea D-CRN simulator. 
/// Its purpose it to take a variety of plaintext source files such as .csv or .rs and compile a reaction network, 
/// which may be simulated by the [MARlea_engine](https://github.com/nadaso8/MARlea_engine) module.

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/csv.pest"]
struct CSVparser;