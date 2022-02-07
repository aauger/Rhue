#[derive(Debug)]
pub enum RuleType {
    Replace,
    Print,
    Input,
}

#[derive(Debug)]
pub struct Rule<'a> {
    pub lhs: &'a str,
    pub rhs: &'a str,
    pub rule_type: RuleType,
}

impl<'a> Rule<'a> {
    pub const fn new(lhs: &'a str, rhs: &'a str, rule_type: RuleType) -> Self {
        Self { lhs, rhs, rule_type }
    }
}
