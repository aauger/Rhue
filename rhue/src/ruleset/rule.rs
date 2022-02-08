#[derive(Debug)]
pub enum RuleType {
    Replace,
    Print,
    Input,
}

#[derive(Debug)]
pub struct Rule {
    pub lhs: String,
    pub rhs: String,
    pub rule_type: RuleType,
}

impl Rule {
    pub fn new(lhs: &str, rhs: &str, rule_type: RuleType) -> Self {
        Self { lhs: lhs.to_owned(), rhs: rhs.to_owned(), rule_type }
    }
}
