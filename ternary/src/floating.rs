use crate::{trits::Trit, word::Word, WORD_LEN};

/// The fundamental ternary floating point type.
///
/// This type has no need for a sign bit or bias, due to balanced
/// ternary already being
/// a) balanced
/// b) signed.
///
/// This means we can represent our mantissa and exponent transparently.
/// Suppose we have an exponent of size 6 and mantissa of size 21.
///
/// Exponent  Mantissa
/// +----+ +-------------------+
/// T110T1 10001T10010T101T10111
///  |         |
/// -137   3520429789
///
/// An exponent of size 6 gives an accuracy in between IEEE-754 single precision
/// and IEEE-754 double precision.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Floating(Word);

const EXPONENT_SIZE: u64 = 6;
const MANTISSA_SIZE: u64 = WORD_LEN as u64 - EXPONENT_SIZE;
const EXPONENT: u64 = (1 << (EXPONENT_SIZE * 2)) - 1;
const MANTISSA: u64 = (1 << (MANTISSA_SIZE * 2)) - 1;
// Masking on a [`Floating`] gives just the exponent
const EXPONENT_MASK: u64 = EXPONENT << (2 * MANTISSA_SIZE);
// Masking on a [`Floating`] gives just the mantissa
const MANTISSA_MASK: u64 = !EXPONENT_MASK;

const_assert_eq!(EXPONENT, 0b111111111111);

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Mantissa(Word);
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Exponent(Word);

impl From<Floating> for (Mantissa, Exponent) {
    fn from(value: Floating) -> Self {
        let arr: [Trit; 27] = value.0.into();
        let mut mantissa: [Trit; 27] = Word::ZERO.into();
        let mut exponent: [Trit; 27] = Word::ZERO.into();
        for (i, idx) in (0..(MANTISSA_SIZE as usize)).rev().enumerate() {
            mantissa[i] = arr[idx];
        }
        for (i, idx) in (0..(EXPONENT_SIZE as usize)).rev().enumerate() {
            exponent[i] = arr[idx + MANTISSA_SIZE as usize];
        }
        (Mantissa(mantissa.into()), Exponent(exponent.into()))
        /*
        let exponent = Word(
            value.0.num() >> (2 * MANTISSA_SIZE)
                | (((Word::ZERO.0) << (2 * MANTISSA_SIZE)) & WORD_BIT_MASK),
        );
        let mantissa =
            Word(((value.0.num() & MANTISSA) | (Word::ZERO.0 & EXPONENT_MASK)) & WORD_BIT_MASK);
        (Mantissa(mantissa), Exponent(exponent))
        */
    }
}

impl From<(Mantissa, Exponent)> for Floating {
    fn from(value: (Mantissa, Exponent)) -> Self {
        let (Mantissa(mantissa), Exponent(exponent)) = value;
        let mantissa: [Trit; 27] = mantissa.into();
        let exponent: [Trit; 27] = exponent.into();
        let mut arr: [Trit; 27] = Word::ZERO.into();
        for i in 0..(MANTISSA_SIZE as usize) {
            arr[i] = mantissa[i];
        }
        for i in 0..(EXPONENT_SIZE as usize) {
            arr[i + MANTISSA_SIZE as usize] = exponent[i]
        }
        Floating(arr.into())
        /*
        let (Mantissa(Word(mantissa)), Exponent(Word(exponent))) = value;
        let exponent = (exponent & EXPONENT) << (2 * MANTISSA_SIZE);
        Floating(Word(exponent | mantissa))
        */
    }
}

impl From<Floating> for f64 {
    fn from(value: Floating) -> Self {
        let (mantissa, exponent): (Mantissa, Exponent) = value.into();
        let exponent: isize = exponent.0.into();
        let mantissa: [Trit; 27] = mantissa.0.into();

        let mut mantissa_acc: f64 = 0.;
        // We know the decimal for the mantissa is after the first digit
        for (i, idx) in (0..MANTISSA_SIZE).enumerate() {
            // i is for the power we take our float to
            // idx is for the index into the mantissa
            // At:
            // - 0: Trit is simply multiplied by 1
            // - -x: Trit is multipled by 3^(-x)
            let pow: f64 = 3f64.powi(-(i as i32));
            let trit = match mantissa[idx as usize] {
                Trit::NOne => -1.,
                Trit::Zero => 0.,
                Trit::POne => 1.,
            };

            mantissa_acc += pow * trit;
        }

        mantissa_acc * 3f64.powf(exponent as f64)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        floating::{Exponent, Floating, Mantissa},
        word::Word,
    };

    #[test]
    fn convert() {
        let mantissa: Mantissa = Mantissa(3141592.into());
        let exponent: Exponent = Exponent(20.into());
        let float: Floating = (mantissa, exponent).into();
        let tuple: (Mantissa, Exponent) = float.into();
        assert_eq!(tuple, (mantissa, exponent));
        // let word: Word = "00111T000000001010T111T1T00".parse().unwrap();
        // assert_eq!(word, float.0);
    }

    #[test]
    fn conversion() {
        let mantissa: Word = 1.into();
        let exponent: Word = 0.into();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, 1.0, epsilon = f64::EPSILON);

        let mantissa: Word = 1.into();
        let exponent: Word = (-1).into();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, 1.0 / 3.0, epsilon = f64::EPSILON);

        let mantissa: Word = (-1).into();
        let exponent: Word = (-1).into();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, -1.0 / 3.0, epsilon = f64::EPSILON);

        let mantissa: Word = (-1).into();
        let exponent: Word = (-3).into();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, -1.0 / 27.0, epsilon = f64::EPSILON);

        let mantissa: Word = "1TTT0110T000T111T11001".parse().unwrap();
        let exponent: Word = (1).into();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, std::f64::consts::PI, epsilon = 0.00000001);
    }
}
