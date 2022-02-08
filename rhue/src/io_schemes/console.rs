use std::io::{self, BufRead};
use crate::ruleset::rule_engine::RuleFuncs;

pub struct ConsoleScheme;

impl RuleFuncs for ConsoleScheme {
    fn print(output: &str) {
        println!("{}", output);
    }

    fn input(prompt: &str) -> String {
        if !prompt.is_empty() {
            println!("{}", prompt);
        }
        let mut input = String::new();
        io::stdin()
            .lock()
            .read_line(&mut input)
            .expect("We should have had an input here");
        input
    }
}