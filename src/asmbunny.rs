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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    // Cpy -> Jnz
    Cpy { source: Source, register: Source },
    // Inc -> Dec
    Inc { register: Source },
    // Dec -> Inc
    Dec { register: Source },
    // Jnz -> Cpy
    Jnz { source: Source, offset: Source },
    // Tgl -> Inc
    Tgl { offset: Source },
}

impl std::str::FromStr for Instruction {
    type Err = AsmError;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        Ok(match &tokens[..] {
            &["cpy", source, register] => {
                let source: Source = source.parse()?;
                let register = register.parse()?;
                Instruction::Cpy { source, register }
            }
            &["inc", register] => {
                let register = register.parse()?;
                Instruction::Inc { register }
            }
            &["dec", register] => {
                let register = register.parse()?;
                Instruction::Dec { register }
            }
            &["jnz", source, offset] => {
                let source = source.parse()?;
                let offset = offset.parse()?;
                Instruction::Jnz { source, offset }
            }
            &["tgl", offset] => {
                let offset = offset.parse()?;
                Instruction::Tgl { offset }
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

    pub fn get_instruction(&self, pos: i64) -> Option<Instruction> {
        if pos < 0 {
            return None;
        }
        if pos as usize >= self.instructions.len() {
            return None;
        }
        self.instructions.get(pos as usize).cloned()
    }

    pub fn get_value(&mut self, source: &Source) -> i64 {
        match source {
            Source::Constant { value } => *value,
            Source::Register { id } => *self.registers.entry(id.clone()).or_insert(0),
        }
    }

    pub fn set_value(&mut self, source: &Source, value: i64) {
        match source {
            Source::Constant { .. } => { /* ignore setting to a constant */ }
            Source::Register { id } => {
                self.registers.insert(id.clone(), value);
            }
        }
    }

    pub fn step_turbo<F: Fn(&mut Self) -> Option<bool>>(&mut self, speed_patch: F) -> bool {
        if let Some(ret) = speed_patch(self) {
            ret
        } else {
            self.step()
        }
    }

    pub fn step(&mut self) -> bool {
        let inst = self.get_instruction(self.ic);
        if inst.is_none() {
            return false;
        }

        match inst.unwrap() {
            Instruction::Cpy { source, register } => {
                let value = self.get_value(&source);
                self.set_value(&register, value);
            }
            Instruction::Inc { register } => {
                let value = self.get_value(&register);
                self.set_value(&register, value + 1);
            }
            Instruction::Dec { register } => {
                let value = self.get_value(&register);
                self.set_value(&register, value - 1);
            }
            Instruction::Jnz { source, offset } => {
                let value = self.get_value(&source);
                let ofs = self.get_value(&offset);
                if value != 0 {
                    self.ic += ofs;
                    return true;
                }
            }
            Instruction::Tgl { offset } => {
                let ofs = self.get_value(&offset);

                if let Some(inst) = self.get_instruction(self.ic + ofs) {
                    let new_inst: Instruction = match inst {
                        Instruction::Cpy { source, register } => Instruction::Jnz {
                            source: source.clone(),
                            offset: register.clone(),
                        },
                        Instruction::Inc { register } => Instruction::Dec {
                            register: register.clone(),
                        },
                        Instruction::Dec { register } => Instruction::Inc {
                            register: register.clone(),
                        },
                        Instruction::Jnz { source, offset } => Instruction::Cpy {
                            source: source.clone(),
                            register: offset.clone(),
                        },
                        Instruction::Tgl { offset } => Instruction::Inc {
                            register: offset.clone(),
                        },
                    };
                    self.instructions[(self.ic + ofs) as usize] = new_inst;
                }
            }
        }
        self.ic += 1;
        true
    }
}
