use crate::ruleset::rule::{Rule, RuleType};
use rand::{thread_rng, Rng};
use std::{collections::HashMap};

const DEBUG: bool = false;

pub trait RuleFuncs {
    fn print(output: &str);
    fn input(prompt: &str) -> String;
}

pub struct RuleEngine<T: RuleFuncs> {
    pub rules: HashMap<String, Vec<Rule>>,
    pub program: String,
    pub stub: T
}

impl<T: RuleFuncs> RuleEngine<T>
{
    pub fn new(rule_list: Vec<Rule>, program: String, stub: T) -> Self {
        let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();
        for rule in rule_list {
            if rules.contains_key(&rule.lhs) {
                if let Some(vec) = rules.get_mut(&rule.lhs) {
                    vec.push(rule);
                }
            } else {
                rules.insert(rule.lhs.to_owned(), vec![rule]);
            }
        }
        return Self {
            rules,
            program,
            stub
        };
    }

    pub fn evaluate(&mut self) -> &str {
        let mut rand = thread_rng();
        loop {
            // Show debug output when this is run with debug mode.
            if DEBUG {
                <T as RuleFuncs>::print(&self.program);
            }
            // Verify current program state has some lhs in it, or return final evaluation
            if !self.rules.keys().any(|k| self.program.contains(k)) {
                break &self.program;
            }
            // Evaluate `lhs`s found in program text
            for (k, v) in &self.rules {
                if self.program.contains(k) {
                    let rule = &v[rand.gen_range(0..v.len())];
                    match rule.rule_type {
                        RuleType::Replace => {
                            self.program = self.program.replacen(&rule.lhs, &rule.rhs, 1);
                        }
                        RuleType::Print => {
                            <T as RuleFuncs>::print(&rule.rhs);
                            self.program = self.program.replacen(&rule.lhs, "", 1);
                        }
                        RuleType::Input => {
                            let input = <T as RuleFuncs>::input(&rule.rhs);
                            self.program = self.program.replacen(&rule.lhs, &input, 1);
                        }
                    }
                }
            }
        }
    }
}
