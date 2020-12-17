use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
type Result<T> = std::result::Result<T, AsmError>;

#[derive(Debug, Snafu)]
pub enum AsmError {
    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Invalid instruction '{}'", data))]
    ParseInstruction { data: String },
}

#[derive(Debug, Clone)]
pub enum Source {
    Constant { value: i64 },
    Register { id: String },
}

impl std::str::FromStr for Source {
    type Err = AsmError;
    fn from_str(s: &str) -> Result<Self> {
        Ok(if let Ok(value) = s.parse::<i64>() {
            Source::Constant { value }
        } else {
            Source::Register { id: s.to_string() }
        })
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Cpy { source: Source, register: String },
    Inc { register: String },
    Dec { register: String },
    Jnz { source: Source, offset: i64 },
}

impl std::str::FromStr for Instruction {
    type Err = AsmError;

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
                return Err(AsmError::ParseInstruction {
                    data: s.to_string(),
                })
            }
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub ic: i64,
    pub registers: HashMap<String, i64>,
    pub instructions: Vec<Instruction>,
}

impl State {
    pub fn from_instructions(instructions: Vec<Instruction>) -> Self {
        State {
            instructions,
            ..Default::default()
        }
    }

    pub fn get_value(&mut self, source: &Source) -> i64 {
        match source {
            Source::Constant { value } => *value,
            Source::Register { id } => *self.registers.entry(id.clone()).or_insert(0),
        }
    }

    pub fn step(&mut self, inst: &Instruction) {
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
