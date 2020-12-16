use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

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

    #[snafu(display("Invalid instruction '{}'", data))]
    ParseInstruction { data: String },
}

#[derive(Debug, Clone)]
enum Source {
    Constant { value: i64 },
    Register { id: String },
}

impl std::str::FromStr for Source {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(if let Ok(value) = s.parse::<i64>() {
            Source::Constant { value }
        } else {
            Source::Register { id: s.to_string() }
        })
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Cpy { source: Source, register: String },
    Inc { register: String },
    Dec { register: String },
    Jnz { source: Source, offset: i64 },
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        Ok(match &tokens[..] {
            &["cpy", source, register] => {
                let source: Source = source.parse()?;
                let register = register.to_string();
                Instruction::Cpy { source, register }
            }
            &["inc", register] => {
                let register = register.to_string();
                Instruction::Inc { register }
            }
            &["dec", register] => {
                let register = register.to_string();
                Instruction::Dec { register }
            }
            &["jnz", source, offset] => {
                let source = source.parse()?;
                let offset: i64 = offset.parse().context(ParseInt {
                    data: offset.to_string(),
                })?;
                Instruction::Jnz { source, offset }
            }
            _ => {
                return Err(Error::ParseInstruction {
                    data: s.to_string(),
                })
            }
        })
    }
}

#[derive(Debug, Clone, Default)]
struct State {
    ic: i64,
    registers: HashMap<String, i64>,
}

impl State {
    fn get_value(&mut self, source: &Source) -> i64 {
        match source {
            Source::Constant { value } => *value,
            Source::Register { id } => *self.registers.entry(id.clone()).or_insert(0),
        }
    }

    fn step(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Cpy { source, register } => {
                let value = self.get_value(source);
                self.registers.insert(register.clone(), value);
            }
            Instruction::Inc { register } => {
                *self.registers.entry(register.clone()).or_insert(0) += 1;
            }
            Instruction::Dec { register } => {
                *self.registers.entry(register.clone()).or_insert(0) -= 1;
            }
            Instruction::Jnz { source, offset } => {
                let value = self.get_value(source);
                if value != 0 {
                    self.ic += offset;
                    return;
                }
            }
        }
        self.ic += 1;
    }
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("data/day12/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let mut state: State = Default::default();
    while (state.ic >= 0) && (state.ic < instructions.len() as i64) {
        let inst = &instructions[state.ic as usize];
        state.step(inst);
    }

    println!("Part 1: {:#?}", state);

    let mut state: State = Default::default();
    state.registers.insert("c".to_string(), 1);
    while (state.ic >= 0) && (state.ic < instructions.len() as i64) {
        let inst = &instructions[state.ic as usize];
        state.step(inst);
    }

    println!("Part 2: {:#?}", state);

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
