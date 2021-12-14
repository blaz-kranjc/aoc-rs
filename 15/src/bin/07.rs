use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Context};

#[derive(Debug, Clone)]
enum Value {
    Constant(u16),
    Register(String),
}

impl FromStr for Value {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse() {
            Ok(Value::Constant(v))
        } else {
            Ok(Value::Register(s.to_owned()))
        }
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Id(Value),
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
    LeftShift(Value, Value),
    RightShift(Value, Value),
}

impl FromStr for Expression {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = |s| Value::from_str(s);
        let parts = s.split(' ').collect::<Vec<_>>();
        if parts.len() == 1 {
            return Ok(Expression::Id(parse(s)?));
        }
        if parts[0] == "NOT" {
            return Ok(Expression::Not(parse(parts[1])?));
        }
        match parts[1] {
            "AND" => Ok(Expression::And(parse(parts[0])?, parse(parts[2])?)),
            "OR" => Ok(Expression::Or(parse(parts[0])?, parse(parts[2])?)),
            "LSHIFT" => Ok(Expression::LeftShift(parse(parts[0])?, parse(parts[2])?)),
            "RSHIFT" => Ok(Expression::RightShift(parse(parts[0])?, parse(parts[2])?)),
            _ => bail!("Unknown operation"),
        }
    }
}

fn evaluate(instructions: &mut HashMap<String, Expression>, value: &Value) -> anyhow::Result<u16> {
    let result = match value {
        &Value::Constant(v) => v,
        Value::Register(r) => {
            let exp = instructions.remove(r).context("Unknown register")?;
            let value = match exp {
                Expression::Id(v) => evaluate(instructions, &v)?,
                Expression::Not(v) => !evaluate(instructions, &v)?,
                Expression::And(l, r) => evaluate(instructions, &l)? & evaluate(instructions, &r)?,
                Expression::Or(l, r) => evaluate(instructions, &l)? | evaluate(instructions, &r)?,
                Expression::LeftShift(l, r) => {
                    evaluate(instructions, &l)? << evaluate(instructions, &r)?
                }
                Expression::RightShift(l, r) => {
                    evaluate(instructions, &l)? >> evaluate(instructions, &r)?
                }
            };
            instructions.insert(r.clone(), Expression::Id(Value::Constant(value)));
            value
        }
    };
    Ok(result)
}

fn read_register(
    instructions: &mut HashMap<String, Expression>,
    register: &str,
) -> anyhow::Result<u16> {
    evaluate(instructions, &Value::Register(register.to_owned()))
}

fn main() {
    let mut connections = aoc::get_input(15, 7)
        .trim()
        .split('\n')
        .map(|s| {
            let mut parts = s.split(" -> ");
            let exp = Expression::from_str(parts.next().unwrap()).unwrap();
            let reg = parts.next().unwrap().to_owned();
            (reg, exp)
        })
        .collect::<HashMap<_, _>>();
    let mut connections_copy = connections.clone();

    let a_direct = read_register(&mut connections, "a").unwrap();
    println!("Part 1: {}", a_direct);

    connections_copy.insert("b".to_owned(), Expression::Id(Value::Constant(a_direct)));
    let a_updated = read_register(&mut &mut connections_copy, "a").unwrap();
    println!("Part 2: {}", a_updated);
}
