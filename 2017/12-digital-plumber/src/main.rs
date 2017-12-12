use std::collections::HashSet;

fn visit_from(start: usize, graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>) {
    for &id in graph[start].iter() {
        if visited.contains(&id) { continue; }
        visited.insert(id);
        visit_from(id, graph, visited);
    }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|l| {
        let toks = l.split("<->").collect::<Vec<&str>>();
        toks[1].trim().split(",").map(|t| t.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>()
}

fn main() {
    let graph = parse_input(include_str!("input").trim());
    let mut visited = HashSet::<usize>::new();
    visited.insert(0);
    visit_from(0, &graph, &mut visited);
    println!("{}", visited.len());
}

#[test]
fn test_code() {
    let graph = parse_input(include_str!("input-simple").trim());
    let mut visited = HashSet::<usize>::new();
    visited.insert(0);
    visit_from(0, &graph, &mut visited);
    assert_eq!(visited.len(), 6);
}
