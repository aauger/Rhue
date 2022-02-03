use std::collections::HashMap;
use super::rule::Rule;

pub struct RuleEngine<'a> {
    pub rules: HashMap<&'a str, Vec<Rule<'a>>>,
}

impl<'a> RuleEngine<'a> {
    pub fn new(rule_list: Vec<Rule<'a>>) -> Self {
        let mut rule_map: HashMap<&'a str, Vec<Rule<'a>>> = HashMap::new();

        for rule in rule_list {
            if rule_map.contains_key(rule.lhs)
            {
                let result = rule_map.get_mut(rule.lhs).unwrap();
                result.push(rule);
            } else {
                rule_map.insert(rule.lhs, vec![rule]);
            }
        }

        Self { rules: rule_map }
    }
}
