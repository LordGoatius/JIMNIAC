use std::fmt::Display;

#[derive(Debug)]
pub struct TritParseErr(pub i64);

impl Display for TritParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Trit Value: {}", self.0)
    }
}

#[derive(Debug)]
pub enum StackError {
    StackOverflow,
    StackUnderflow,
}

impl Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::StackOverflow  => write!(f, "Stack Overflow"),
            StackError::StackUnderflow => write!(f, "Stack Underflow")
        }
    }
}
