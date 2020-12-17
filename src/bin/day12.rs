use snafu::{ResultExt, Snafu};

use aoc2016::asmbunny::{AsmError, Instruction, State};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Asmbunny error: {}", source))]
    Asm { source: AsmError },
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("data/day12/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse().context(Asm))
        .collect::<Result<_>>()?;

    let mut state = State::from_instructions(instructions.clone());
    while (state.ic >= 0) && (state.ic < instructions.len() as i64) {
        let inst = &instructions[state.ic as usize];
        state.step(inst);
    }

    println!("Part 1: {:#?}", state.registers["a"]);

    let mut state: State = State::from_instructions(instructions.clone());
    state.registers.insert("c".to_string(), 1);
    while (state.ic >= 0) && (state.ic < instructions.len() as i64) {
        let inst = &instructions[state.ic as usize];
        state.step(inst);
    }

    println!("Part 2: {:#?}", state.registers["a"]);

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
