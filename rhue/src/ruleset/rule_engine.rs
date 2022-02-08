use crate::ruleset::rule::{Rule, RuleType};
use rand::{thread_rng, Rng};
use std::{collections::HashMap, marker::PhantomData};

const DEBUG: bool = false;

pub trait EngineIoScheme {
    fn print(output: &str);
    fn input(prompt: &str) -> String;
}

pub struct RuleEngine<T: EngineIoScheme> {
    pub rules: HashMap<String, Vec<Rule>>,
    pub program: String,
    _stub: PhantomData<T>
}

impl<T: EngineIoScheme> RuleEngine<T>
{
    pub fn new(rule_list: Vec<Rule>, program: String) -> Self {
        let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();
        for rule in rule_list {
            if rules.contains_key(&rule.lhs) {
                if let Some(vec) = rules.get_mut(&rule.lhs) {
                    vec.push(rule);
                }
            } else {
                rules.insert(rule.lhs.clone(), vec![rule]);
            }
        }
        return Self {
            rules,
            program,
            _stub: PhantomData
        };
    }

    pub fn evaluate(&mut self) -> &str {
        let mut rand = thread_rng();
        loop {
            // Show debug output when this is run with debug mode.
            if DEBUG {
                T::print(&self.program);
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
                            T::print(&rule.rhs);
                            self.program = self.program.replacen(&rule.lhs, "", 1);
                        }
                        RuleType::Input => {
                            let input = T::input(&rule.rhs);
                            self.program = self.program.replacen(&rule.lhs, &input, 1);
                        }
                    }
                }
            }
        }
    }
}
