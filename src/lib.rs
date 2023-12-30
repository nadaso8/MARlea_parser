/// Author: Marceline Sorensen 
/// Email: nadaso8th@gmail.com 
/// Date: 12/24/2023
/// 
/// # Description
/// This is the parser module for the Marlea D-CRN simulator. 
/// Its purpose it to take a variety of plaintext source files such as .csv or .rs and compile a reaction network, 
/// which may be simulated by the [MARlea_engine](https://github.com/nadaso8/MARlea_engine) module.

use std::{path::Path, collections::{HashMap, HashSet}, io::{BufRead, Read}};

use pest::{Parser, iterators::{Pair, Pairs}};
use pest_derive::Parser;

use marlea_engine::trial::reaction_network::{ReactionNetwork, self, solution::{Solution, Name, Count}, reaction::Reaction};

// derive parsers 
#[derive(Parser)]
#[grammar = "grammars/csv.pest"]
struct CSVparser;

impl CSVparser {
    /// recursively parse csv parser rules into a reaction network 
    fn to_reaction_network(pairs: Pairs<'_, Rule>) -> ReactionNetwork {
        let reactions: HashSet<Reaction> = HashSet::new();
        let species_counts:HashMap<Name, Count> = HashMap::new();

        for pair in pairs {
            
        }

        return ReactionNetwork::new(reactions, Solution{ species_counts});
    }
}


pub enum MarleaParserError {
    ParseFailed(String),
    UnsupportedExt(String),
    InvalidFile(String),
}

// object containing any settings needed or relevant to the marlea parser 
struct marlea_parser;

impl marlea_parser {
    pub fn new() -> Self{
        Self
    }

    /// Parses a reaction network and solution from a variety of file types 
    pub fn parse(path: &Path) -> Result<ReactionNetwork,MarleaParserError> {
        // default catch all error state for parser
        match path.extension() {
            Some("csv") => {

            },
            None => Result::Err(MarleaParserError::InvalidFile(format!("provided  Path: {} \ndid not contain an extension or does not exist"))),
            
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::CSVparser;

    #[test]
    fn csv() {
        let input = {"fibonacci.call => setup.call,1,
        setup.done => calculate.call,1,
        ,,
        setup.call => destruct + next_value + setup.call,1,
        destruct + 2 next_value => next_value + destruct ,10000,
        destruct + last_value => destruct,10000,
        destruct + current_value => destruct ,10000,
        destruct + setup.call => destruct ,10000,
        ,,
        next_value.less_than.2.index.1 + setup.call.not.index.1 => destruct.done.partial.0,10000,
        2 next_value.less_than.2.index.0 => next_value.less_than.2.index.1,10000,
        2 next_value.less_than.2.index.1 => next_value.less_than.2.index.1,10000,
        destruct => destruct + next_value.less_than.2.index.0,1,
        2 next_value + next_value.less_than.2.index.0 => 2 next_value,10000,
        2 next_value + next_value.less_than.2.index.1 => 2 next_value,10000,
        2 setup.call.not.index.0 => setup.call.not.index.1,10000,
        2 setup.call.not.index.1 => setup.call.not.index.1,10000,
        destruct => destruct + setup.call.not.index.0,1,
        setup.call + setup.call.not.index.0 => setup.call,10000,
        setup.call + setup.call.not.index.1 => setup.call,10000,
        ,,
        current_value.not.index.1 + last_value.not.index.1 => destruct.done.partial.1,10000,
        2 current_value.not.index.0 => current_value.not.index.1,10000,
        2 current_value.not.index.1 => current_value.not.index.1,10000,
        destruct => destruct + current_value.not.index.0,1,
        current_value + current_value.not.index.0 => current_value ,10000,
        current_value + current_value.not.index.1 => current_value ,10000,
        2 last_value.not.index.1 => last_value.not.index.1,10000,
        2 last_value.not.index.0 => last_value.not.index.1,10000,
        destruct => destruct + last_value.not.index.0,1,
        last_value + last_value.not.index.0 => last_value,10000,
        last_value + last_value.not.index.1 => last_value,10000,
        ,,
        destruct.done.partial.0 + destruct.done.partial.1 => destruct.done,1,
        2 destruct.done.partial.0 => destruct.done.partial.1,10000,
        2 destruct.done.partial.1 => destruct.done.partial.1,10000,
        2 destruct.done => destruct.done,10000,
        destruct.done + destruct => destruct.done,10000,
        ,,
        destruct.not.index.1 => setup.done,1,
        2 destruct.not.index.1 => destruct.not.index.1,10000,
        2 destruct.not.index.0 => destruct.not.index.1,10000,
        destruct.done => destruct.done + destruct.not.index.0,1,
        destruct + destruct.not.index.1 => destruct,10000,
        destruct + destruct.not.index.0 => destruct,10000,
        setup.done + destruct.done => setup.done,10000,
        ,,
        2 calculate.call => calculate.call,10000,
        calculate.call + calculate.done => calculate.call ,10000,
        calculate.call => index.check,1,
        index.check + calculate.call => index.check,10000,
        ,,
        2 index.check => index.check,10000,
        index.check + index => current_value.convert,1,
        current_value.convert + index.check => current_value.convert,10000,
        index.check + index.not.index.1 => calculate.return,1,
        2 index.not.index.1 => index.not.index.1,10000,
        2 index.not.index.0 => index.not.index.1,10000,
        index.check => index.check + index.not.index.0,1,
        index + index.not.index.0 => index,10000,
        index + index.not.index.1 => index,10000,
        calculate.return + index.check => calculate.return,10000,
        ,,
        2 current_value.convert => current_value.convert,10000,
        current_value.convert + current_value => last_value + current_value.convert,10000,
        current_value.convert + current_value.not.index.1 => next_value.convert ,1,
        2 current_value.not.index.1 => current_value.not.index.1,10000,
        2 current_value.not.index.0 => current_value.not.index.1,10000,
        current_value.convert => current_value.convert + current_value.not.index.0,1,
        current_value + current_value.not.index.0 => current_value ,10000,
        current_value + current_value.not.index.1 => current_value ,10000,
        ,,
        2 next_value.convert => next_value.convert,10000,
        next_value.convert + next_value => next_value.swap + next_value.convert,10000,
        next_value.convert + next_value.not.index.1 => next_value.split,1,
        2 next_value.not.index.1 => next_value.not.index.1,10000,
        2 next_value.not.index.0 => next_value.not.index.1,10000,
        next_value.convert => next_value.not.index.0 + next_value.convert,1,
        next_value + next_value.not.index.1 => next_value,10000,
        next_value + next_value.not.index.0 => next_value,10000,
        next_value.split + next_value.convert => next_value.split ,10000,
        ,,
        2 next_value.split => next_value.split,10000,
        next_value.split + next_value.swap => next_value + current_value + next_value.split,10000,
        next_value.split + next_value.swap.not.index.1 => last_value.convert ,1,
        2 next_value.swap.not.index.1 => next_value.swap.not.index.1,10000,
        2 next_value.swap.not.index.0 => next_value.swap.not.index.1,10000,
        next_value.split => next_value.swap.not.index.0 + next_value.split,1,
        next_value.swap + next_value.swap.not.index.1 => next_value.swap,10000,
        next_value.swap + next_value.swap.not.index.0 => next_value.swap,10000,
        last_value.convert + next_value.split => last_value.convert,10000,
        ,,
        2 last_value.convert => last_value.convert,10000,
        last_value.convert + last_value => next_value + last_value.convert,10000,
        last_value.convert + last_value.not.index.1 => index.check + last_value.convert ,1,
        2 last_value.not.index.1 => last_value.not.index.1,10000,
        2 last_value.not.index.0 => last_value.not.index.1,10000,
        last_value.convert => last_value.convert + last_value.not.index.0,1,
        last_value + last_value.not.index.0 => last_value,10000,
        last_value + last_value.not.index.1 => last_value,10000,
        index.check + last_value.convert => index.check ,10000,
        ,,
        2 calculate.return => calculate.return ,10000,
        calculate.return + current_value => return + calculate.return,10000,
        calculate.return + current_value.not.index.1 => calculate.done,1,
        2 current_value.not.index.1 => current_value.not.index.1,10000,
        2 current_value.not.index.0 => current_value.not.index.1,10000,
        calculate.return => calculate.return + current_value.not.index.0,1,
        current_value + current_value.not.index.0 => current_value,10000,
        current_value + current_value.not.index.1 => current_value,10000,
        fibonacci.call,1,
        index,20,
        "};

        //print!("{}", input);
        match CSVparser::parse(crate::Rule::reaction_network, input) {
            Ok(result) => {
                print!("{}\n", result);
                for pair in result{
                    let rule = match pair.as_rule() {
                        crate::Rule::coefficient => "coefficient",
                        crate::Rule::comma_delimiter => "comma_delimiter", 
                        crate::Rule::comment => "comment",
                        crate::Rule::EOI => "end",
                        crate::Rule::fat_arrow_delimiter => "fat_arrow_delimiter",
                        crate::Rule::name => "name",
                        crate::Rule::new_line_delimiter => "new_line_delimiter",
                        crate::Rule::plus_delimiter => "plus_delimiter",
                        crate::Rule::products => "products", 
                        crate::Rule::reactants => "reactants",
                        crate::Rule::reaction => "reaction",
                        crate::Rule::reaction_rate => "reaction_rate",
                        crate::Rule::reaction_network => "reaction_network",
                        crate::Rule::space_delimiter => "space_delimiter",
                        crate::Rule::species_count => "species_count",
                        crate::Rule::term => "term",
                    };
                    let text = pair.as_str();
                    println!("rule: {} matched: {}", rule, text);
                }
            },
            Err(msg) => {
                print!("{} \n\n\n", msg);
                panic!("failed to parse")
            }
        };
    }
}