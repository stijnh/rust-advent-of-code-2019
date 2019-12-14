pub use anyhow::{Context, Error};
pub use itertools::{enumerate, zip};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::default::Default;
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

pub fn default<T: Default>() -> T {
    T::default()
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
