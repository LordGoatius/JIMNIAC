use crate::{tryte::Tryte, word::Word};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Register(Word);

impl Register {
    pub fn replace_word(&mut self, word: Word) {
        self.0 = word;
    }

    pub fn replace_tryte(&mut self, tryte: Tryte) {
        self.0 = [tryte, Tryte::default(), Tryte::default()].into();
    }
}

impl From<Register> for Tryte {
    fn from(value: Register) -> Self {
        value.into()
    }
}

impl From<Word> for Register {
    fn from(value: Word) -> Self {
        Register(value)
    }
}
