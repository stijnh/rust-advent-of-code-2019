use std::fs::File;
use std::io::{BufRead, BufReader};
pub use anyhow::{Error, Context};
pub use itertools::{enumerate};

macro_rules! iff {
    ($cond:expr, $a:expr, $b:expr) => {
        if $cond {
            $a
        } else {
            $b
        }
    }
}

pub type Result<T=(), E=Error> = std::result::Result<T, E>;

pub(crate) fn read_input(filename: &str) -> Result<Vec<String>> {
    let path = format!("inputs/{}", filename);
    let f = File::open(&path)?;
    let lines = BufReader::new(f)
        .lines()
        .collect::<Result<_, _>>()
        .with_context(|| format!("failed to find {}", path))?;
    Ok(lines)
}