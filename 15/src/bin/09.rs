use std::{cmp, collections::HashMap};

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<i32>,
    n_columns: usize,
}

impl Grid {
    fn new(n_rows: usize, n_columns: usize) -> Grid {
        Grid {
            data: vec![0; n_rows * n_columns],
            n_columns,
        }
    }

    fn to_index(&self, r: usize, c: usize) -> usize {
        r * self.n_columns + c
    }

    fn get(&self, r: usize, c: usize) -> i32 {
        self.data[self.to_index(r, c)]
    }

    fn set(&mut self, r: usize, c: usize, v: i32) {
        let index = self.to_index(r, c);
        self.data[index] = v;
    }

    fn n_columns(&self) -> usize {
        self.n_columns
    }
}

fn to_connection_map(conns: &[(String, String, i32)]) -> Grid {
    let mut curr = 0;
    let mut nodes = HashMap::new();
    for (n1, n2, _) in conns {
        if !nodes.contains_key(&n1 as &str) {
            nodes.insert(n1.clone(), curr);
            curr += 1;
        }
        if !nodes.contains_key(&n2 as &str) {
            nodes.insert(n2.clone(), curr);
            curr += 1;
        }
    }
    let mut grid = Grid::new(nodes.len(), nodes.len());
    for (n1, n2, d) in conns {
        let i1 = *nodes.get(n1).unwrap();
        let i2 = *nodes.get(n2).unwrap();
        grid.set(i1, i2, *d);
        grid.set(i2, i1, *d);
    }
    grid
}

fn weight(graph: &Grid, permutation: &[usize]) -> i32 {
    permutation.windows(2).map(|e| graph.get(e[0], e[1])).sum()
}

fn next_permutation(permutation: &mut [usize]) -> bool {
    let mut i = (permutation.len() - 2) as i64;
    while i >= 0 && permutation[i as usize + 1] <= permutation[i as usize] {
        i -= 1;
    }
    if i >= 0 {
        let mut j = permutation.len() - 1;
        while permutation[j] <= permutation[i as usize] {
            j -= 1;
        }
        permutation.swap(i as usize, j);
        permutation[i as usize + 1..].reverse();
        true
    } else {
        false
    }
}

fn min_max_path(graph: &Grid) -> (i32, i32) {
    let mut permutation = (0..graph.n_columns()).collect::<Vec<_>>();
    let mut min = weight(&graph, &permutation);
    let mut max = weight(&graph, &permutation);
    loop {
        if next_permutation(&mut permutation) {
            let current = weight(graph, &permutation);
            min = cmp::min(min, current);
            max = cmp::max(max, current);
        } else {
            break;
        }
    }
    (min, max)
}

fn main() {
    let conns = aoc::get_input(15, 9)
        .trim()
        .split('\n')
        .map(|l| {
            let mut it = l.split(' ').step_by(2);
            (
                it.next().unwrap().to_owned(),
                it.next().unwrap().to_owned(),
                it.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let graph = to_connection_map(&conns);

    let (min, max) = min_max_path(&graph);
    println!("Part 1: {}", min);
    println!("Part 2: {}", max);
}
