use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum StackError {}

impl Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Stack Error")
        }
    }
}

impl Error for StackError {}
