use std::fs::File;

use snafu::{ResultExt, Snafu};

use aoc2016::map::{Map, MapError, MapTile};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Map parsing error: {}", source))]
    ParseMap { source: MapError },
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

impl MapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Safe),
            '^' => Some(Tile::Trap),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Safe => ".",
                Tile::Trap => "^",
            }
        )
    }
}

fn generate(start_row: &[bool], rows: usize) -> usize {
    let width = start_row.len();
    let mut last_row = start_row.to_vec();
    let mut new_row = Vec::with_capacity(width);
    let mut n_safe = start_row.iter().filter(|v| !*v).count();

    for _ in 1..rows {
        for j in 0..width {
            let left = if j > 0 { last_row[j - 1] } else { false };
            let center = last_row[j];
            let right = if j < width - 1 {
                last_row[j + 1]
            } else {
                false
            };

            let current = match (left, center, right) {
                (true, true, false) => true,
                (false, true, true) => true,
                (true, false, false) => true,
                (false, false, true) => true,
                _ => false,
            };

            if !current {
                n_safe += 1;
            }

            new_row.push(current);
        }

        last_row = new_row;
        new_row = Vec::with_capacity(width);
    }

    n_safe
}

fn main() -> Result<()> {
    let first_row: Vec<bool> = std::fs::read_to_string("data/day18/input")
        .context(Io)?
        .trim()
        .chars()
        .map(|c| c == '^')
        .collect();

    println!("Part 1: Got {} safe tiles", generate(&first_row, 40));
    println!("Part 2: Got {} safe tiles", generate(&first_row, 400000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
