use crate::ruleset::rule_engine::EngineIoScheme;
use std::io::{self, BufRead};

pub struct ConsoleScheme;

impl EngineIoScheme for ConsoleScheme {
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
