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
    let instructions: Vec<Instruction> = std::fs::read_to_string("data/day23/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse().context(Asm))
        .collect::<Result<_>>()?;

    // let instructions: Vec<Instruction> = "cpy 2 a\ntgl a\ntgl a\ntgl a\ncpy 1 a\ndec a\ndec a"
    //     .lines()
    //     .map(|l| l.parse().context(Asm))
    //     .collect::<Result<_>>()?;

    let mut state = State::from_instructions(instructions.clone());
    state.registers.insert("a".to_string(), 7);
    while let Some(_inst) = state.get_instruction(state.ic) {
        // println!("{:3} {:?} {:?}", state.ic, inst, state.registers);
        state.step();
    }
    println!("Part 1: {:#?}", state.registers["a"]);

    let mut state = State::from_instructions(instructions.clone());
    state.registers.insert("a".to_string(), 12);
    while let Some(_inst) = state.get_instruction(state.ic) {
        // println!("{:3} {:?} {:?}", state.ic, inst, state.registers);
        state.step_turbo(|s| {
            if s.ic == 5 {
                let slow: Vec<Instruction> = ["inc a", "dec c", "jnz c -2", "dec d", "jnz d -5"]
                    .into_iter()
                    .map(|i| i.parse().unwrap())
                    .collect();

                for (i, si) in slow.into_iter().enumerate() {
                    if let Some(pi) = s.get_instruction(5 + i as i64) {
                        if si != pi {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                // we have now determined that the next 5 instructions match.
                // run fast program instead.
                let a = s.registers["a"];
                let c = s.registers["c"];
                let d = s.registers["d"];
                // println!("TURBO: a = {} + {} * {} ", a, c, d);

                s.registers.insert("a".to_string(), a + c * d);
                s.registers.insert("c".to_string(), 0);
                s.registers.insert("d".to_string(), 0);

                s.ic += 5;

                return Some(true);
            }
            None
        });
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
