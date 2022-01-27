#[derive(Debug)]
pub struct Rule<'a>{
    pub lhs: &'a str,
    pub rhs: &'a str
}

impl<'a> Rule<'a> {
    pub fn new(lhs: &'a str, rhs: &'a str) -> Self {
        Rule { lhs, rhs }
    }
}