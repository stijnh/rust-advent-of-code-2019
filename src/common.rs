pub use anyhow::{Context, Error};
pub use itertools::{all, any, enumerate, zip, Itertools as _};
use std::default::Default;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
pub use thiserror::Error;

macro_rules! iff {
    ($cond:expr, $a:expr, $b:expr) => {
        if $cond {
            $a
        } else {
            $b
        }
    };
}

macro_rules! format_err {
    ($($msg:tt)*) => {
        $crate::common::StringError(format!($($msg)*))
    };
}

macro_rules! bail {
    ($obj:expr) => {
        bail!("{}", $obj);
    };
    ($($msg:tt)*) => {
        return std::result::Result::Err(format_err!($($msg)*).into());
    }
}

#[derive(Error, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[error("{0}")]
pub struct StringError(pub String);

#[inline(always)]
pub fn default<T: Default>() -> T {
    T::default()
}

#[inline(always)]
pub fn sum<I>(iter: I) -> I::Item
where
    I: IntoIterator,
    I::Item: Sum,
{
    iter.into_iter().sum()
}

#[inline(always)]
pub fn map<I, F, B>(fun: F, iter: I) -> impl Iterator<Item = B>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> B,
{
    iter.into_iter().map(fun)
}

pub fn cycle<T: Clone>(val: T) -> impl Iterator<Item = T> {
    Some(val).into_iter().cycle()
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub(crate) fn read_input(filename: &str) -> Result<Vec<String>> {
    let path = format!("inputs/{}", filename);
    let f = File::open(&path).with_context(|| format!("failed to open {}", path))?;
    BufReader::new(f)
        .lines()
        .collect::<Result<_, _>>()
        .with_context(|| format!("error while reading {}", path))
}
