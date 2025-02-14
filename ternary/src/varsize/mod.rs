use std::{hash::Hash, ops::{Deref, DerefMut}};

use crate::trits::*;

pub mod binops;
pub mod unops;
pub mod tritops;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Varsize<const N: usize>(pub [Trit; N]);

impl<const N: usize> Default for Varsize<N> {
    fn default() -> Self {
        Varsize([Trit::Zero; N])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarAddResult<const N: usize> {
    pub carry: Trit,
    pub result: Varsize<N>,
}

//=== Impl Word ===//

impl<const N: usize> Varsize<N> {
    fn abs(value: Self) -> Self {
        if value < Varsize::default() {
            -value
        } else {
            value
        }
    }
}

//== Helper Traits ==//

impl<const N: usize> Deref for Varsize<N> {
    type Target = [Trit; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for Varsize<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> From<[Trit; N]> for Varsize<N> {
    fn from(value: [Trit; N]) -> Self {
        Varsize(value)
    }
}

impl<const N: usize> From<Varsize<N>> for isize {
    fn from(value: Varsize<N>) -> Self {
        value
            .iter()
            .enumerate()
            .map(|(i, trit)| match trit {
                Trit::NOne => -isize::pow(3, i as u32),
                Trit::Zero => 0,
                Trit::POne => isize::pow(3, i as u32),
            })
            .sum()
    }
}

impl<const N: usize> PartialOrd for Varsize<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Varsize<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_isize: isize = (*self).into();

        let other_isize: isize = (*other).into();
        self_isize.cmp(&other_isize)
    }
}
