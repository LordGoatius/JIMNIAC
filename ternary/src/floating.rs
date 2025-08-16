use std::ops::{Add, Mul, Neg, Shl, Shr, Sub};

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
        // the first digit is always at the 20th index
        // a mantissa of 1 should translate to a 1 with 20 0s
        for i in 0..(MANTISSA_SIZE as usize) {
            mantissa[i] = arr[i];
        }
        for i in 0..(EXPONENT_SIZE as usize) {
            exponent[i] = arr[i + MANTISSA_SIZE as usize];
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
        for (i, idx) in (0..MANTISSA_SIZE).rev().enumerate() {
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

impl Neg for Mantissa {
    type Output = Mantissa;

    fn neg(self) -> Self::Output {
        Mantissa(-self.0)
    }
}

impl Mul for Mantissa {
    type Output = Mantissa;

    fn mul(self, rhs: Self) -> Self::Output {
        Mantissa(self.0 * rhs.0)
    }
}

impl Add for Mantissa {
    type Output = Mantissa;

    fn add(self, rhs: Self) -> Self::Output {
        Mantissa(self.0 + rhs.0)
    }
}

impl Add for Exponent {
    type Output = Exponent;

    fn add(self, rhs: Self) -> Self::Output {
        Exponent(self.0 + rhs.0)
    }
}

impl Exponent {
    fn valid_exponent(self) -> bool {
        let arr: [Trit; 27] = self.0.into();
        for i in 6..WORD_LEN {
            if arr[i] != Trit::Zero {
                return false;
            }
        }
        true
    }

    fn leading_zeroes(self) -> usize {
        let arr: [Trit; 27] = self.0.into();
        let mut count = 0;
        for i in (0..(EXPONENT_SIZE as usize)).rev() {
            if let Trit::Zero = arr[i] {
                count += 1;
            } else {
                return count;
            }
        }
        count
    }
}

impl Mantissa {
    fn leading_zeroes(self) -> usize {
        let arr: [Trit; 27] = self.0.into();
        let mut count = 0;
        for i in (0..WORD_LEN).rev() {
            if let Trit::Zero = arr[i] {
                count += 1;
            } else {
                return count;
            }
        }
        count
    }

    fn trailing_zeroes(self) -> usize {
        let arr: [Trit; 27] = self.0.into();
        let mut count = 0;
        for i in 0..WORD_LEN {
            if let Trit::Zero = arr[i] {
                count += 1;
            } else {
                return count;
            }
        }
        count
    }
}

impl Neg for Floating {
    type Output = Floating;

    fn neg(self) -> Self::Output {
        let (mantissa, exponent): (Mantissa, Exponent) = self.into();
        let mantissa = -mantissa;
        (mantissa, exponent).into()
    }
}

impl Add for Floating {
    type Output = Floating;

    fn add(self, rhs: Self) -> Self::Output {
        let (mantissa_0, exponent_0): (Mantissa, Exponent) = self.into();
        let (mantissa_1, exponent_1): (Mantissa, Exponent) = rhs.into();

        let exponent_num_0: isize = exponent_0.0.into();
        let exponent_num_1: isize = exponent_1.0.into();

        // The diff is how much bigger the lhs exponent is from the rhs exponent.
        // If it is greater than 21 then there is no reason to add them, and we
        // shoud only take one. If the difference is negative, 1 is bigger, and
        // if the difference is positive, 0 is larger.
        let diff_exp = exponent_num_0 - exponent_num_1;
        // The rhs is too small to matter
        if diff_exp >= MANTISSA_SIZE as isize {
            return self;
        // The lhs is too small to matter
        } else if diff_exp <= -(MANTISSA_SIZE as isize) {
            return rhs;
        }

        // TODO: At some point I'll simplify this, but for now
        // that's for the LLVM optimizer to handle
        match diff_exp.signum() {
            -1 => {
                // The exp_1/rhs is larger than the exp_0/lhs
                // right shift mant_0 and add to exp_0 until
                // the exp matches
                let mantissa_0 = Mantissa(mantissa_0.0 >> (diff_exp.abs().cast_unsigned()));
                let mantissa = mantissa_0 + mantissa_1;
                let leading = mantissa.leading_zeroes();
                let diff = EXPONENT_SIZE as isize - leading.cast_signed();
                let exponent: Word = (exponent_num_1 + diff).into();
                let mantissa = if diff < 0 {
                    Mantissa(mantissa.0 << diff.abs().cast_unsigned())
                } else {
                    Mantissa(mantissa.0 >> diff.abs().cast_unsigned())
                };
                (mantissa, Exponent(exponent)).into()
            }
            0 => {
                let leading = mantissa_0.leading_zeroes().max(mantissa_1.leading_zeroes()) as isize;
                let mantissa = mantissa_1 + mantissa_0;
                let leading_add = mantissa.leading_zeroes() as isize;
                // the difference between the previous leading zeroes and the new
                // leading zeros is how much we need to change the exponent
                let diff: Word = (leading - leading_add).into();
                let diff_int = leading - leading_add;
                let exponent = exponent_0.0 + diff;
                // Exponent is 1
                // Exponent is 2
                let mantissa = if diff_int < 0 {
                    // needs to shift left
                    Mantissa(mantissa.0 << (diff_int.abs().cast_unsigned()))
                } else if diff_int > 0 {
                    // needs to shift right
                    Mantissa(mantissa.0 >> (diff_int.abs().cast_unsigned()))
                } else {
                    mantissa
                };
                let float: Floating = (mantissa, Exponent(exponent)).into();
                float
            }
            1 => {
                // The mantissa_0/lhs is larger than the mantissa_1/rhs
                // right shift rhs and add to exp_1 until
                // the exp matches
                let mantissa_1 = Mantissa(mantissa_1.0 >> (diff_exp.abs().cast_unsigned()));
                let mantissa = mantissa_1 + mantissa_0;
                let leading = mantissa.leading_zeroes();
                let diff = EXPONENT_SIZE as isize - leading.cast_signed();
                let exponent: Word = (exponent_num_0 + diff).into();
                let mantissa = if diff < 0 {
                    Mantissa(mantissa.0 << diff.abs().cast_unsigned())
                } else {
                    Mantissa(mantissa.0 >> diff.abs().cast_unsigned())
                };
                (mantissa, Exponent(exponent)).into()
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl Sub for Floating {
    type Output = Floating;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Floating {
    type Output = Floating;

    fn mul(self, rhs: Self) -> Self::Output {
        // Multiply mantissa, add exponents, an normalize
        let (mantissa_0, exponent_0) = self.into();
        let (mantissa_1, exponent_1) = rhs.into();

        eprintln!("m0 {}", mantissa_0.0);
        eprintln!("m1 {}", mantissa_1.0);
        // This doesn't work really. I need to find out exactly how to know
        // how much to shift each one right until it doesn't overflow
        let trailing_0 = mantissa_0.trailing_zeroes();
        let trailing_1 = mantissa_1.trailing_zeroes();

        eprintln!("t0 {}", trailing_0);
        eprintln!("t1 {}", trailing_1);

        let temp_0 = mantissa_0.0 >> trailing_0;
        let temp_1 = mantissa_1.0 >> trailing_1;
        let max_leading = Mantissa(temp_0)
            .leading_zeroes()
            .min(Mantissa(temp_1).leading_zeroes());

        let mut mantissa = Mantissa(temp_0 * temp_1);
        let leading_diff = max_leading - mantissa.leading_zeroes();
        eprintln!("mm {} {}", mantissa.0, leading_diff);

        // We need to shift `trailing_min` up, which is equivalent to adding
        // trailing min to the exponent. We can then normalize to achieve
        // greater accuracy then shifting the mantissa now;
        let trailing_word: Exponent = Exponent((trailing_0.min(trailing_1) - leading_diff).cast_signed().into());
        eprintln!("tr {}", trailing_word.0);
        let mut exponent = exponent_0 + exponent_1 + trailing_word;

        let leading = mantissa.leading_zeroes();
        eprintln!("ld {} {}", leading, 6 - leading);

        let change = 6 - leading.cast_signed();

        let shift: (fn(Word, usize) -> Word, Word) = match change.signum() {
            // leading > 6, we want to shift left
            -1 => (<Word as Shl<usize>>::shl, Word::NONE),
            // leading < 6, we want to shift right
            1 => (<Word as Shr<usize>>::shr, Word::PONE),
            0 => {
                return (mantissa, exponent).into();
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        };

        eprintln!("m {} e {}", mantissa.0, exponent.0);
        while mantissa.leading_zeroes() != 6 {
            mantissa = Mantissa(shift.0(mantissa.0, 1));
            exponent = Exponent(exponent.0 + shift.1);
            eprintln!("m {} e {}", mantissa.0, exponent.0);
        }

        eprintln!();

        (mantissa, exponent).into()
    }
}

impl Floating {
    const MAX: Floating = Floating(Word::MAX);
}

#[cfg(test)]
pub mod tests {
    use std::f64;

    use crate::{
        floating::{Exponent, Floating, Mantissa},
        word::Word,
    };

    #[test]
    fn test_add() {
        let mantissa: Word = "100000000000000000000".parse().unwrap();
        let exponent: Word = 1.into();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();

        let prod = floating + -floating;
        let double: f64 = prod.into();

        assert_relative_eq!(double, 0.0, epsilon = f64::EPSILON);

        let mantissa_0: Word = "100000000000000000000".parse().unwrap();
        let exponent_0: Word = 1.into();
        let three: Floating = (Mantissa(mantissa_0), Exponent(exponent_0)).into();
        assert_relative_eq!(
            <Floating as Into<f64>>::into(three),
            3.0,
            epsilon = f64::EPSILON
        );

        let mantissa_1: Word = "100000000000000000000".parse().unwrap();
        let exponent_1: Word = 2.into();
        let nine: Floating = (Mantissa(mantissa_1), Exponent(exponent_1)).into();
        assert_relative_eq!(
            <Floating as Into<f64>>::into(nine),
            9.0,
            epsilon = f64::EPSILON
        );

        let twelve = three + nine;
        let double: f64 = twelve.into();

        assert_relative_eq!(double, 12.0, epsilon = f64::EPSILON);

        let sum = three + three;
        let double: f64 = sum.into();

        assert_relative_eq!(double, 6.0, epsilon = f64::EPSILON);

        // 17
        let mantissa_0: Word = "1T0T00000000000000000".parse().unwrap();
        let exponent_0: Word = 3.into();
        let flt_0: Floating = (Mantissa(mantissa_0), Exponent(exponent_0)).into();
        assert_relative_eq!(
            <Floating as Into<f64>>::into(flt_0),
            17.0,
            epsilon = f64::EPSILON
        );

        // 12
        let mantissa_1: Word = "110000000000000000000".parse().unwrap();
        let exponent_1: Word = 2.into();
        let flt_1: Floating = (Mantissa(mantissa_1), Exponent(exponent_1)).into();
        assert_relative_eq!(
            <Floating as Into<f64>>::into(flt_1),
            12.0,
            epsilon = f64::EPSILON
        );

        let sum: f64 = (flt_0 + flt_1).into();
        assert_relative_eq!(sum, 29.0, epsilon = f64::EPSILON);

        let sum: f64 = (flt_1 + flt_0).into();
        assert_relative_eq!(sum, 29.0, epsilon = f64::EPSILON);

        let sum: f64 = (flt_0 + -flt_1).into();
        assert_relative_eq!(sum, 5.0, epsilon = f64::EPSILON);

        let sum: f64 = (flt_1 + -flt_0).into();
        assert_relative_eq!(sum, -5.0, epsilon = f64::EPSILON);

        let sum: f64 = (flt_1 - flt_0).into();
        assert_relative_eq!(sum, -5.0, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_mul() {
        let mantissa_0: Word = "100000000000000000000".parse().unwrap();
        let exponent_0: Word = 1.into();
        let three: Floating = (Mantissa(mantissa_0), Exponent(exponent_0)).into();
        assert_relative_eq!(
            <Floating as Into<f64>>::into(three),
            3.0,
            epsilon = f64::EPSILON
        );

        let mantissa_1: Word = "100000000000000000000".parse().unwrap();
        let exponent_1: Word = 2.into();
        let nine: Floating = (Mantissa(mantissa_1), Exponent(exponent_1)).into();
        assert_relative_eq!(
            <Floating as Into<f64>>::into(nine),
            9.0,
            epsilon = f64::EPSILON
        );

        let flt: f64 = (three * three).into();
        assert_relative_eq!(flt, 9.0, epsilon = f64::EPSILON);
        let flt: f64 = (nine * nine).into();
        assert_relative_eq!(flt, 81.0, epsilon = f64::EPSILON);
        let flt: f64 = (nine * three).into();
        assert_relative_eq!(flt, 27.0, epsilon = f64::EPSILON);

        // 12
        let mantissa_0: Word = "110000000000000000000".parse().unwrap();
        let exponent_0: Word = 2.into();
        let twelve: Floating = (Mantissa(mantissa_0), Exponent(exponent_0)).into();
        assert_relative_eq!(
            <Floating as Into<f64>>::into(twelve),
            12.0,
            epsilon = f64::EPSILON
        );

        let mantissa: Word = "10011T111T000T0110TTT".parse().unwrap();
        let exponent: Word = (1).into();
        let pi: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = pi.into();
        assert_relative_eq!(double, f64::consts::PI, epsilon = 0.00000001);

        let pi_mul_3 = pi * three;
        let val: f64 = pi_mul_3.into();
        assert_relative_eq!(val, f64::consts::PI * 3., epsilon = 0.00000001);

        let pi_mul_9 = pi * nine;
        let val: f64 = pi_mul_9.into();
        // My PI isn't fully accurate yet so unfortunately this needs slightly
        // less accuracy :(
        assert_relative_eq!(val, f64::consts::PI * 9., epsilon = 0.0000001);

        let pi_mul_12 = pi * twelve;
        let val: f64 = pi_mul_12.into();
        // My PI isn't fully accurate yet so unfortunately this needs slightly
        // less accuracy :(
        assert_relative_eq!(val, f64::consts::PI * 12., epsilon = 0.0000001);

        let val: f64 = (pi * pi).into();
        assert_relative_eq!(val, f64::consts::PI * f64::consts::PI, epsilon = 0.0000001);
    }

    #[test]
    fn convert() {
        let mantissa: Mantissa = Mantissa(3141592.into());
        let exponent: Exponent = Exponent(20.into());
        let float: Floating = (mantissa, exponent).into();
        let tuple: (Mantissa, Exponent) = float.into();
        assert_eq!(tuple, (mantissa, exponent));
    }

    #[test]
    fn conversion() {
        let mantissa: Word = "100000000000000000000".parse().unwrap();
        let exponent: Word = "000000".parse().unwrap();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, 1.0, epsilon = f64::EPSILON);

        let mantissa: Word = "100000000000000000000".parse().unwrap();
        let exponent: Word = "00000T".parse().unwrap();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, 1.0 / 3.0, epsilon = f64::EPSILON);

        let mantissa: Word = "T00000000000000000000".parse().unwrap();
        let exponent: Word = "00000T".parse().unwrap();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, -1.0 / 3.0, epsilon = f64::EPSILON);

        let mantissa: Word = "T00000000000000000000".parse().unwrap();
        let exponent: Word = "0000T0".parse().unwrap();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, -1.0 / 27.0, epsilon = f64::EPSILON);

        let mantissa: Word = "T00000000000000000000".parse().unwrap();
        let exponent: Word = "0000T0".parse().unwrap();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let floating: Floating = -floating;
        let double: f64 = floating.into();
        assert_relative_eq!(double, 1.0 / 27.0, epsilon = f64::EPSILON);

        let mantissa: Word = "10011T111T000T0110TTT".parse().unwrap();
        let exponent: Word = (1).into();
        let floating: Floating = (Mantissa(mantissa), Exponent(exponent)).into();
        let double: f64 = floating.into();
        assert_relative_eq!(double, std::f64::consts::PI, epsilon = 0.00000001);
    }

    #[test]
    fn leading_trailing_zeroes() {
        assert_eq!(27, Mantissa(Word::ZERO).leading_zeroes());
        assert_eq!(26, Mantissa(Word::PONE).leading_zeroes());
        assert_eq!(0, Mantissa(Word::MIN).leading_zeroes());
        let mantissa: Word = "TTT0110T000T111T11001".parse().unwrap();
        assert_eq!(6, Mantissa(mantissa).leading_zeroes());
        assert_eq!(0, Mantissa(mantissa).trailing_zeroes());
        let mantissa: Word = "000000000000000010000000000".parse().unwrap();
        assert_eq!(16, Mantissa(mantissa).leading_zeroes());
        assert_eq!(10, Mantissa(mantissa).trailing_zeroes());
        let exponent: Word = "100000".parse().unwrap();
        assert_eq!(0, Exponent(exponent).leading_zeroes());
        let exponent: Word = "100".parse().unwrap();
        assert_eq!(3, Exponent(exponent).leading_zeroes());
    }
}
