
#[derive(Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
}

enum ParserError {
    InvalidOperator,
}

impl Operator {
    fn from_char(c: char) -> Result<Operator, ParserError> {
        match c {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Sub),
            '*' => Ok(Operator::Mul),
            _ => Err(ParserError::InvalidOperator),
        }
    }
}
