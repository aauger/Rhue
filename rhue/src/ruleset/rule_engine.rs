use crate::ruleset::rule::{Rule, RuleType};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

const DEBUG: bool = false;

pub struct RuleEngine<'a> {
    pub rules: HashMap<&'a str, Vec<Rule<'a>>>,
    pub program: String,
}

impl<'a> RuleEngine<'a> {
    pub fn new(rule_list: Vec<Rule<'a>>, program: String) -> Self {
        let mut rules: HashMap<&'a str, Vec<Rule<'a>>> = HashMap::new();
        for rule in rule_list {
            if rules.contains_key(rule.lhs) {
                if let Some(vec) = rules.get_mut(rule.lhs) {
                    vec.push(rule);
                }
            } else {
                rules.insert(rule.lhs, vec![rule]);
            }
        }

        Self { rules, program }
    }

    pub fn evaluate(&mut self) -> &str {
        let mut rand = thread_rng();
        loop {
            if DEBUG {
                println! {"DEBUG: {}", self.program};
            }
            if !self.rules.keys().any(|k| self.program.contains(k)) {
                break &self.program;
            }
            for (k, v) in &self.rules {
                if self.program.contains(k) {
                    let rule = &v[rand.gen_range(0..v.len())];
                    match rule.rule_type {
                        RuleType::Replace => {
                            self.program = self.program.replacen(rule.lhs, rule.rhs, 1)
                        }
                        RuleType::Print => {
                            print!("{}", rule.rhs);
                            self.program = self.program.replacen(rule.lhs, "", 1);
                        }
                        RuleType::Input => todo!(),
                    }
                }
            }
        }
    }
}
