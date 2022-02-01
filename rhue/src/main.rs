use std::{env::args, fs::File, io::{Read, self}, result::Result};

mod ruleset;
use crate::ruleset::rule::{Rule, RuleType};
use rand::{thread_rng, prelude::ThreadRng};

fn main() -> Result<(), io::Error> {
    let rand: ThreadRng = thread_rng();
    let args: Vec<String> = args().skip(1).collect();
    let rule_file_path = &args[0];
    let program_file_path = &args[1];

    let mut rule_file = File::open(rule_file_path)?;
    let mut program_file = File::open(program_file_path)?;

    let mut rule_file_text = String::new();
    let mut program_file_text = String::new();
    
    rule_file.read_to_string(&mut rule_file_text)?;
    program_file.read_to_string(&mut program_file_text)?;

    let rules = rule_file_text.split("\n").map(|rt| {
        let rule_split: Vec<&str> = rt.split("::=").collect();
        Rule::new(rule_split[0], rule_split[1], RuleType::Replace)
    });

    println!("Loaded ruleset:\n");
    for rule in rules {
        println!("lhs: {}, rhs: {}", rule.lhs, rule.rhs);
    }

    Ok(())
}
