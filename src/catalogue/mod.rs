use std::fmt::{Display, Formatter, Result};

mod advent_of_code;

pub trait Problem {
    fn get_solver(&self) -> Vec<impl Solver>;
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait Solver {
    fn solve(&self) -> Solution;
    fn get_info(&self) -> &str;
}

pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Solution::*;
        match self {
            I8(x) => x.fmt(f),
            I16(x) => x.fmt(f),
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            I128(x) => x.fmt(f),
            ISize(x) => x.fmt(f),
            U8(x) => x.fmt(f),
            U16(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            U64(x) => x.fmt(f),
            U128(x) => x.fmt(f),
            USize(x) => x.fmt(f),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, ISize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, USize);

pub fn problem_from_str(s: &str) -> Option<impl Problem> {
    let mut parts = s.split(':');

    let kind = parts.next()?;
    let problem = parts.next()?;

    if parts.next().is_some() {
        return None;
    }

    match kind {
        advent_of_code::IDENTIFIER => advent_of_code::AdventOfCodeProblem::from_str(problem),
        _ => None,
    }
}
