use std::{
    env::args,
    fs::File,
    io::{self, Read},
    result::Result,
};

mod ruleset;
use ruleset::rule_engine::RuleEngine;
use ruleset::{
    rule::{Rule, RuleType}
};

mod io_schemes;

use crate::io_schemes::console::ConsoleScheme;

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

    let rules = rule_file_text
        .lines()
        .map(|rt| {
            let rule_split: Vec<&str> = rt.split("::=").collect();
            let lhs = rule_split[0].trim();
            let rhs = rule_split[1].trim();

            match rhs {
                a if a.starts_with(":::") => Rule::new(lhs, &rhs[3..], RuleType::Input),
                a if a.starts_with('~') => Rule::new(lhs, &rhs[1..], RuleType::Print),
                _ => Rule::new(lhs, rhs, RuleType::Replace),
            }
        })
        .collect::<Vec<Rule>>();

    let mut rule_engine: RuleEngine<ConsoleScheme> = RuleEngine::new(rules, program_file_text);
    let evaluated_text = rule_engine.evaluate();

    println!("{}", evaluated_text);
    Ok(())
}
