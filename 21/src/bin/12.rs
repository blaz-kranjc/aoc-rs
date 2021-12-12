use std::collections::{HashMap, HashSet};

use anyhow::bail;

fn to_edge_map(edges: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    for e in edges {
        if !result.contains_key(&e.0) {
            result.insert(e.0.clone(), vec![]);
        }
        if !result.contains_key(&e.1) {
            result.insert(e.1.clone(), vec![]);
        }
        result
            .get_mut(&e.0)
            .iter_mut()
            .for_each(|v| v.push(e.1.clone()));
        result
            .get_mut(&e.1)
            .iter_mut()
            .for_each(|v| v.push(e.0.clone()));
    }
    result
}

fn count_paths<'a>(
    graph: &'a HashMap<String, Vec<String>>,
    current: &'a str,
    visited: &mut HashSet<&'a str>,
    revisited: bool,
) -> u64 {
    if current == "end" {
        return 1;
    }

    let mut result = 0;
    if !current.chars().any(char::is_uppercase) {
        visited.insert(current);
    }
    for e in graph.get(current).unwrap() {
        let did_visit = visited.contains(&e as &str);
        if !did_visit {
            result += count_paths(graph, e, &mut visited.clone(), revisited);
        } else if e != "start" && !revisited {
            result += count_paths(graph, e, &mut visited.clone(), true);
        }
    }
    result
}

fn main() {
    let edges = aoc::get_input(21, 12)
        .trim()
        .split('\n')
        .map(|s| {
            let parts = s.split('-').collect::<Vec<_>>();
            if parts.len() != 2 {
                bail!("Wrong number of parts")
            } else {
                Ok((parts[0].to_owned(), parts[1].to_owned()))
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("Wrong input");
    let graph = to_edge_map(edges);

    println!(
        "Part 1: {}",
        count_paths(&graph, "start", &mut HashSet::new(), true)
    );
    println!(
        "Part 2: {}",
        count_paths(&graph, "start", &mut HashSet::new(), false)
    );
}
