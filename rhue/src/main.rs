use std::{env::args, fs::File, io::{Read, self}, result::Result};

mod ruleset;
use crate::ruleset::rule::Rule;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().skip(1).collect();
    let rule_file_path = &args[0];
    let program_file_path = &args[1];

    let mut rule_file = File::open(rule_file_path)?;
    let mut program_file = File::open(program_file_path)?;

    let mut rule_file_text = String::new();
    let mut program_file_text = String::new();
    
    rule_file.read_to_string(&mut rule_file_text)?;
    program_file.read_to_string(&mut program_file_text)?;

    let test_rule: Rule = Rule::new("hello", "world");

    println!("TEST RULE: ");
    println!("{}: {}", test_rule.lhs, test_rule.rhs);

    println!("RULESET: ");
    println!("{}\n\n", rule_file_text);
    println!("PROGRAM: ");
    println!("{}", program_file_text);

    Ok(())
}
