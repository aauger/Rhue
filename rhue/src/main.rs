use std::{
    env::args,
    fs::File,
    io::{self, Read},
    result::Result,
};

mod ruleset;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use ruleset::rule::{Rule, RuleType};
use ruleset::rule_engine::RuleEngine;

const DEBUG: bool = true;

fn main() -> Result<(), io::Error> {
    let mut rand: ThreadRng = thread_rng();
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

            let rule = match rhs {
                a if a.starts_with(":::") => Rule::new(lhs, &rhs[3..], RuleType::Input),
                a if a.starts_with('~') => Rule::new(lhs, &rhs[1..], RuleType::Print),
                _ => Rule::new(lhs, rhs, RuleType::Replace),
            };

            rule
        })
        .collect::<Vec<Rule>>();

    let rule_engine = RuleEngine::new(rules);
    let mut program = program_file_text;

    loop {
        if DEBUG {
            println!{"DEBUG: {}", program};
        }
        if !rule_engine.rules.keys().any(|k| program.contains(k)) {
            break;
        }
        for (k, v) in &rule_engine.rules {
            if program.contains(k) {
                let rule = &v[rand.gen_range(0..v.len())];
                match rule.rule_type {
                    RuleType::Replace => program = program.replacen(rule.lhs, rule.rhs, 1),
                    RuleType::Print => {
                        println!("{}", rule.rhs);
                        program = program.replacen(rule.lhs, "", 1);
                    }
                    RuleType::Input => todo!(),
                }
            }
        }
    }

    println!("{}", program);

    Ok(())
}
