use std::ops::{Deref, DerefMut};

use crate::word::Word;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Register(Word);

impl Deref for Register {
    type Target = Word;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Word> for Register {
    fn from(value: Word) -> Self {
        Register(value)
    }
}
