use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

use anyhow::{bail, Context};

trait Grid {
    fn get(&self, i: usize) -> i8;
    fn len(&self) -> usize;
    fn n_columns(&self) -> usize;

    fn n_rows(&self) -> usize {
        self.len() / self.n_columns()
    }

    fn neighbors(&self, i: usize) -> Vec<usize> {
        let mut result = vec![];
        let to_index = |row: usize, col: usize| row * self.n_columns() + col;
        let (r, c) = (i / self.n_columns(), i % self.n_columns());
        if r > 0 {
            result.push(to_index(r - 1, c));
        }
        if r < self.n_rows() - 1 {
            result.push(to_index(r + 1, c));
        }
        if c > 0 {
            result.push(to_index(r, c - 1));
        }
        if c < self.n_columns() - 1 {
            result.push(to_index(r, c + 1));
        }
        result
    }
}

#[derive(Debug)]
struct BaseGrid {
    data: Vec<i8>,
    n_columns: usize,
}

impl Grid for BaseGrid {
    fn get(&self, i: usize) -> i8 {
        self.data[i]
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn n_columns(&self) -> usize {
        self.n_columns
    }
}

#[derive(Debug)]
struct ExtendedGrid {
    grid: BaseGrid,
    factor: i8,
}

impl Grid for ExtendedGrid {
    fn get(&self, i: usize) -> i8 {
        let (r, c) = (i / self.n_columns(), i % self.n_columns());
        let base_row = r % self.grid.n_rows();
        let base_column = r % self.grid.n_columns();
        let base_index = base_row * self.grid.n_columns + base_column;
        let base = self.grid.get(base_index);

        let row_offset = r / self.grid.n_rows();
        let col_offset = c / self.grid.n_columns();
        let result = base as usize + row_offset + col_offset;
        ((result - 1) % 9 + 1) as i8
    }

    fn len(&self) -> usize {
        self.grid.len() * self.factor.pow(2) as usize
    }

    fn n_columns(&self) -> usize {
        self.grid.n_columns() * self.factor as usize
    }
}

impl FromStr for BaseGrid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .split('\n')
            .map(|r| {
                r.chars()
                    .map(|c| c.to_digit(10).map(|n| n as i8))
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()
            .context("Invalid input characters")?;
        if rows.is_empty() {
            bail!("No rows supplied");
        }
        let n_columns = rows[0].len();
        if rows.iter().skip(1).any(|v| v.len() != n_columns) {
            bail!("Not all rows of the same size");
        }
        Ok(BaseGrid {
            data: rows.into_iter().flatten().collect(),
            n_columns,
        })
    }
}

#[derive(Debug)]
struct Weight(usize, i32);

impl PartialEq for Weight {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl Eq for Weight {}

impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl Ord for Weight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

fn path_length(grid: &impl Grid, from: usize, to: usize) -> Option<i32> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut candidates = BinaryHeap::new();

    distances.insert(from, 0);
    candidates.push(Weight(from, 0));

    while let Some(Weight(node, distance)) = candidates.pop() {
        if node == to {
            return Some(distance);
        }

        if !visited.insert(node) {
            continue;
        }

        for neighbor in grid.neighbors(node) {
            let distance = distance + grid.get(neighbor) as i32;
            let is_shorter = distances
                .get(&neighbor)
                .map_or(true, |&current| distance < current);
            if is_shorter {
                distances.insert(neighbor, distance);
                candidates.push(Weight(neighbor, distance))
            }
        }
    }

    None
}

fn main() {
    let cave = BaseGrid::from_str(aoc::get_input(21, 15).trim()).unwrap();

    println!(
        "Part 1: {:?}",
        path_length(&cave, 0, cave.len() - 1).unwrap()
    );

    let extended = ExtendedGrid {
        grid: cave,
        factor: 5,
    };
    println!(
        "Part 2: {:?}",
        path_length(&extended, 0, extended.len() - 1).unwrap()
    );
}
