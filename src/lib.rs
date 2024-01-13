/// Author: Marceline Sorensen 
/// Email: nadaso8th@gmail.com 
/// Date: 12/24/2023
/// 
/// # Description
/// This is the parser module for the Marlea D-CRN simulator. 
/// Its purpose it to take a variety of plaintext source files such as .csv or .rs and compile a reaction network, 
/// which may be simulated by the [MARlea_engine](https://github.com/nadaso8/MARlea_engine) module.

use std::{path::Path, collections::{HashMap, HashSet}, io::Read, fs::File};

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use marlea_engine::trial::reaction_network::{ReactionNetwork, solution::{Name, Count, Solution}, reaction::{Reaction, term::Term}};

// derive parsers 
#[derive(Parser)]
#[grammar = "grammars/csv.pest"]
struct CSVparser;

impl CSVparser {
    /// gen token stream and parse into a reaction network 
    pub fn as_reaction_network(source: &str) -> Result<ReactionNetwork,MarleaParserError> {
        return match Self::parse(Rule::reaction_network, &source) {
            Ok(mut token_stream) => {
                let mut reactions = HashSet::new();
                let mut species_counts = HashMap::new();        
                let reaction_network = match token_stream.next() {
                    Some(token) => token,
                    None => return Result::Err(MarleaParserError::ParseFailed(format!("Source file was parsed but token stream is empty")))
                };

                for token in reaction_network.into_inner() {
                    match token.as_rule() {
                        Rule::reaction => {
                            // parse reaction token into a reaction object
                            let reaction = match Self::as_reaction(token) {
                                Result::Ok(reaction) => reaction,
                                Result::Err(msg) => return Result::Err(msg)
                            };

                            reactions.insert(reaction.clone());

                            // loop over reactants and products and try to insert any names into species_counts
                            for term in reaction.get_reactants() {
                                species_counts.insert(term.get_species_name().clone(), Count(0));
                            }
                            for term in reaction.get_products() {
                                species_counts.insert(term.get_species_name().clone(), Count(0));    
                            }

                        },
                        Rule::species_count => {
                            // parse species_count token into a (Name, Count) pair 
                            let mut species_count = match Self::as_species_count(token) {
                                Result::Ok(species_count) => species_count,
                                Result::Err(msg) =>  return Result::Err(msg),
                            };
                            
                            // update or insert species (Name, Count) pair
                            species_counts.get_mut(&species_count.0).get_or_insert(&mut species_count.1);
                        },
                        _ => ()
                    };
                }

                Result::Ok(ReactionNetwork::new(reactions, Solution{species_counts}))
            },
            // error if pest fails to match a reaction network token this should catch basically everything and contains the most information back to the user
            Err(msg) => Result::Err(MarleaParserError::ParseFailed(format!("{}", msg)))
        }
    }

    fn as_reaction (token: Pair<'_, Rule>) -> Result<Reaction,MarleaParserError> {
        match token.as_rule() {
            Rule::reaction => {
                let mut reactants = Vec::new();
                let mut products = Vec::new();
                let mut possible_reaction_rate = None;

                for sub_token in token.into_inner() {
                    match sub_token.as_rule() {
                        Rule::reactants => {
                            for reactant_token in sub_token.into_inner() {
                                match Self::as_term(reactant_token) {
                                    Ok(term) => reactants.push(term),
                                    Err(msg) => return Result::Err(msg) 
                                }
                            }
                        },
                        Rule::products => {
                            for product_token in sub_token.into_inner() {
                                match Self::as_term(product_token) {
                                    Ok(term) => products.push(term),
                                    Err(msg) => return Result::Err(msg) 
                                }
                            }
                        }, 
                        Rule::reaction_rate => {
                            possible_reaction_rate = Some(match Self::as_reaction_rate(sub_token) {
                                Ok(reaction_rate) => reaction_rate,
                                Err(msg) => return Result::Err(msg)
                            })
                        },
                        _ => ()
                    }
                }

                match possible_reaction_rate {
                    Some(reaction_rate) => {
                        Result::Ok(Reaction::new(
                            reactants, 
                            products, 
                            reaction_rate.0
                        ))
                    }
                    None => Result::Err(MarleaParserError::ParseFailed(format!("could not find reaction rate in reaction token stream")))
                }
            },
            _ => Result::Err(MarleaParserError::ParseFailed(format!("found unexpected {} token {}, expected reaction token", Self::rule_as_str(token.as_rule()), token.as_str()))),
        }
    }

    fn as_term (token: Pair<'_, Rule>) -> Result<Term,MarleaParserError> {
        match token.as_rule() {
            Rule::term => {
                let mut possible_term: (Option<Name>, Option<Count>) = (None, None);
                for sub_token in token.into_inner() {
                    match sub_token.as_rule() {
                        Rule::name => {
                            let name = match Self::as_name(sub_token){
                                Ok(val) => val,
                                Err(msg) => return Result::Err(msg)
                            };
                            possible_term.0 = Some(name);
                        },
                        Rule::coefficient => {
                            let coefficient = match Self::as_count(sub_token) {
                                Ok(val) => val,
                                Err(msg) => return Result::Err(msg)
                            };
                            possible_term.1 = Some(coefficient);
                        },
                        _ => ()
                    }
                }

                match possible_term {
                    (Some(species_name), Some(coefficient)) => Result::Ok(Term::new(species_name, coefficient)),
                    _ => Result::Err(MarleaParserError::ParseFailed(format!("missing data for term in token stream")))
                }
            },
            _ => Result::Err(MarleaParserError::ParseFailed(format!("found unexpected {} token {}, expected term token", Self::rule_as_str(token.as_rule()), token.as_str()))),
        }
    } 

    fn as_name (token: Pair<'_, Rule>) -> Result<Name,MarleaParserError> {
        match token.as_rule() {
            Rule::name => {
                Result::Ok(Name(token.as_str().to_string()))
            },
            _ => Result::Err(MarleaParserError::ParseFailed(format!("found unexpected {} token {}, expected name token", Self::rule_as_str(token.as_rule()), token.as_str()))),
        }
    } 

    fn as_count (token: Pair<'_, Rule>) -> Result<Count,MarleaParserError> {
        match token.as_rule() {
            Rule::coefficient => {
                if let Ok(count) = token.as_str().parse() {
                    Result::Ok(Count(count))
                } else {
                    // if this error is ever returned you are &$&^%#
                    Result::Err(MarleaParserError::ParseFailed(format!("something has gone seriously wrong at line {} input {}\nUnparseable character discovered ", token.line_col().0 , token.as_str())))
                }
            },
            _ => Result::Err(MarleaParserError::ParseFailed(format!("found unexpected {} token {}, expected coefficient token", Self::rule_as_str(token.as_rule()), token.as_str()))),
        }
    } 

    fn as_reaction_rate (token: Pair<'_, Rule>) -> Result<Count,MarleaParserError> {
        match token.as_rule() {
            Rule::reaction_rate => {
                if let Ok(reaction_rate) = token.as_str().parse() {
                    Result::Ok(Count(reaction_rate))
                } else {
                    // if this error is ever returned you are &$&^%# 
                    Result::Err(MarleaParserError::ParseFailed(format!("something has gone seriously wrong at line {} input {}\nUnparseable character discovered", token.line_col().0 , token.as_str())))
                }
            },
            _ => Result::Err(MarleaParserError::ParseFailed(format!("found unexpected {} token {}, expected reaction rate token", Self::rule_as_str(token.as_rule()), token.as_str()))),
        }
    }
    
    fn as_species_count (token: Pair<'_, Rule>) -> Result<(Name, Count), MarleaParserError> {
        match token.as_rule() {
            Rule::species_count => {
            let mut possible_name = Option::None;
            let mut possible_count = Option::None;

            // known failure modes if multiple name tokens or count tokens present in stream.
            // this should be impossible but will result in the last token of each type in the stream being used.
            for sub_token in token.clone().into_inner() {
                match sub_token.as_rule() {
                    Rule::name => {
                        possible_name = match Self::as_name(sub_token) {
                            Ok(name) => Some(name),
                            Err(msg) => return Result::Err(msg)
                        }
                    }, 
                    Rule::coefficient => {
                        possible_count = match Self::as_count(sub_token) {
                            Ok(count) => Some(count),
                            Err(msg) => return Result::Err(msg)
                        }
                    },
                    _ => ()
                }
            }

            return match (possible_name, possible_count) {
                (Some(name), Some(count)) => Result::Ok((name, count)),
                _ => Result::Err(MarleaParserError::ParseFailed(format!("something has gone seriously wrong\nmissing name or count in {} on line {}", token.as_str(), token.line_col().0)))
            }            
            },
            _ => Result::Err(MarleaParserError::ParseFailed(format!("found unexpected {} token {}, expected species count token", Self::rule_as_str(token.as_rule()), token.as_str()))),            
        }
    }

    pub fn rule_as_str(rule: Rule) -> &'static str {
        match rule {
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
        }
    }
}


pub enum MarleaParserError {
    ParseFailed(String),
    UnsupportedExt(String),
    InvalidFile(String),
}

// object containing any settings needed or relevant to the marlea parser 
pub struct MarleaParser;

impl MarleaParser {
    pub fn new() -> Self{
        Self
    }

    /// Parses a reaction network and solution from a variety of file types 
    pub fn parse(path: &Path) -> Result<ReactionNetwork,MarleaParserError> {
        // match to see if extension exists
        return match path.extension() {
            Some(ext) => {

                // try match to supported extenstion type 
                match ext.to_str() {
                    Some("csv") => {

                        // try to open the file 
                        match File::open(path) {
                            Ok(mut source_file) => {    
                                let mut source_text = String::new();

                                // try to read the file 
                                match source_file.read_to_string(&mut source_text) {
                                    Ok(_) => {
                                        // parse using csv parser 
                                        CSVparser::as_reaction_network(&source_text)
                                    },
                                    Err(_) => Result::Err(MarleaParserError::ParseFailed(format!("failed to read {}" , path.display()))),
                                }
                            },
                            Err(_) => Result::Err(MarleaParserError::ParseFailed(format!("failed to open {}" , path.display()))),
                        }
                    },
                    Some(_) | None => Result::Err(MarleaParserError::UnsupportedExt(format!("provided file {} is not a supported format", path.display() ))),
                }
            },
            None => Result::Err(MarleaParserError::InvalidFile(format!("provided  Path: {} \ndid not contain an extension or does not exist", path.display() ))),
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