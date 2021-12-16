use std::str::FromStr;

use anyhow::{bail, Context};

#[derive(Debug)]
struct Register(usize);

impl FromStr for Register {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "a" {
            Ok(Register(0))
        } else if s == "b" {
            Ok(Register(1))
        } else {
            bail!("Unknown register");
        }
    }
}

#[derive(Debug)]
struct Offset(i32);

impl FromStr for Offset {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Offset(s.parse()?))
    }
}

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        if let Some(cmd) = parts.next() {
            match cmd {
                "hlf" => Ok(Instruction::Half(Register::from_str(
                    parts.next().context("missing register")?,
                )?)),
                "tpl" => Ok(Instruction::Triple(Register::from_str(
                    parts.next().context("missing register")?,
                )?)),
                "inc" => Ok(Instruction::Increment(Register::from_str(
                    parts.next().context("missing register")?,
                )?)),
                "jmp" => Ok(Instruction::Jump(Offset::from_str(
                    parts.next().context("missing offset")?,
                )?)),
                "jie" => {
                    let r = parts.next().expect("missing register");
                    let r = Register::from_str(&r[..r.len() - 1])?;
                    let v = Offset::from_str(parts.next().expect("missing offset"))?;
                    Ok(Instruction::JumpIfEven(r, v))
                }
                "jio" => {
                    let r = parts.next().expect("missing register");
                    let r = Register::from_str(&r[..r.len() - 1])?;
                    let v = Offset::from_str(parts.next().expect("missing offset"))?;
                    Ok(Instruction::JumpIfOne(r, v))
                }
                _ => bail!("Unknown instruction"),
            }
        } else {
            bail!("Missing instruction")
        }
    }
}

fn run(program: &[Instruction], initial: i64) -> i64 {
    let mut index = 0i32;
    let mut registers = [initial, 0];
    while index >= 0 && (index as usize) < program.len() {
        match program[index as usize] {
            Instruction::Half(Register(i)) => {
                registers[i] = registers[i] / 2;
                index += 1;
            }
            Instruction::Triple(Register(i)) => {
                registers[i] = registers[i] * 3;
                index += 1;
            }
            Instruction::Increment(Register(i)) => {
                registers[i] = registers[i] + 1;
                index += 1;
            }
            Instruction::Jump(Offset(off)) => {
                index += off;
            }
            Instruction::JumpIfEven(Register(i), Offset(off)) => {
                if registers[i] % 2 == 0 {
                    index += off;
                } else {
                    index += 1;
                }
            }
            Instruction::JumpIfOne(Register(i), Offset(off)) => {
                if registers[i] == 1 {
                    index += off;
                } else {
                    index += 1;
                }
            }
        }
    }
    registers[1]
}

fn main() {
    let program = aoc::get_input(15, 23)
        .trim()
        .split('\n')
        .map(|v| Instruction::from_str(v).unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", run(&program, 0));
    println!("Part 2: {}", run(&program, 1));
}
