use std::borrow::Borrow;

use anyhow::bail;

#[derive(Debug)]
struct Packet {
    version: u8,
    value: Value,
}

#[derive(Debug)]
enum Value {
    Literal(u64),
    Operator(u8, Vec<Packet>),
}

fn to_bits(c: char) -> anyhow::Result<[bool; 4]> {
    match c {
        '0' => Ok([false, false, false, false]),
        '1' => Ok([false, false, false, true]),
        '2' => Ok([false, false, true, false]),
        '3' => Ok([false, false, true, true]),
        '4' => Ok([false, true, false, false]),
        '5' => Ok([false, true, false, true]),
        '6' => Ok([false, true, true, false]),
        '7' => Ok([false, true, true, true]),
        '8' => Ok([true, false, false, false]),
        '9' => Ok([true, false, false, true]),
        'A' => Ok([true, false, true, false]),
        'B' => Ok([true, false, true, true]),
        'C' => Ok([true, true, false, false]),
        'D' => Ok([true, true, false, true]),
        'E' => Ok([true, true, true, false]),
        'F' => Ok([true, true, true, true]),
        _ => bail!("Invalid character"),
    }
}

fn deserialize_numbers(data: &[bool]) -> u64 {
    let mut result = 0;
    for d in data {
        result = (result << 1) + (*d as u64)
    }
    result
}

fn deserialize_packet(data: &[bool]) -> anyhow::Result<(usize, Packet)> {
    let version = deserialize_numbers(&data[..3]) as u8;
    let packet_type = deserialize_numbers(&data[3..6]) as u8;
    if packet_type == 4 {
        let mut i = 6;
        let mut v = 0;
        while data[i] {
            v = (v << 4) + deserialize_numbers(&data[i + 1..i + 5]);
            i += 5;
        }
        v = (v << 4) + deserialize_numbers(&data[i + 1..i + 5]);
        i += 5;
        Ok((
            i,
            Packet {
                version,
                value: Value::Literal(v),
            },
        ))
    } else {
        let length_id = data[6];
        let mut subpackets = vec![];
        if length_id {
            let n_subpackets = deserialize_numbers(&data[7..7 + 11]) as usize;
            let mut i = 7 + 11;
            while subpackets.len() < n_subpackets {
                let (size, p) = deserialize_packet(&data[i..])?;
                subpackets.push(p);
                i += size;
            }
            Ok((
                i,
                Packet {
                    version,
                    value: Value::Operator(packet_type, subpackets),
                },
            ))
        } else {
            let target_size = 7 + 15 + deserialize_numbers(&data[7..7 + 15]) as usize;
            let mut i = 7 + 15;
            while i < target_size {
                let (size, p) = deserialize_packet(&data[i..])?;
                subpackets.push(p);
                i += size;
            }
            if i != target_size {
                bail!("misaligned packets");
            }
            Ok((
                i,
                Packet {
                    version,
                    value: Value::Operator(packet_type, subpackets),
                },
            ))
        }
    }
}

fn sum_versions(p: &Packet) -> usize {
    return p.version as usize
        + match p.value.borrow() {
            Value::Operator(_, ps) => ps.iter().fold(0, |acc, p| acc + sum_versions(p)),
            _ => 0,
        };
}

fn evaluate(p: &Packet) -> u64 {
    match p.value.borrow() {
        &Value::Literal(v) => v,
        Value::Operator(0, ps) => ps.iter().map(|p| evaluate(p)).sum(),
        Value::Operator(1, ps) => ps.iter().map(|p| evaluate(p)).fold(1, |acc, e| acc * e),
        Value::Operator(2, ps) => ps.iter().map(|p| evaluate(p)).min().unwrap(),
        Value::Operator(3, ps) => ps.iter().map(|p| evaluate(p)).max().unwrap(),
        Value::Operator(5, ps) => (evaluate(&ps[0]) > evaluate(&ps[1])) as u64,
        Value::Operator(6, ps) => (evaluate(&ps[0]) < evaluate(&ps[1])) as u64,
        Value::Operator(7, ps) => (evaluate(&ps[0]) == evaluate(&ps[1])) as u64,
        _ => panic!("Unknown operation"),
    }
}

fn main() {
    let data = aoc::get_input(21, 16)
        .trim()
        .chars()
        .map(|c| to_bits(c).unwrap())
        .flatten()
        .collect::<Vec<_>>();
    let packet = deserialize_packet(&data).expect("Invalid packet").1;

    println!("Part 1: {}", sum_versions(&packet));
    println!("Part 2: {}", evaluate(&packet));
}
