use std::collections::HashSet;

fn visit_from(start: usize, graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>) {
    for &id in graph[start].iter() {
        if visited.contains(&id) { continue; }
        visited.insert(id);
        visit_from(id, graph, visited);
    }
}

fn group_count(graph: &Vec<Vec<usize>>) -> usize {
    let mut visited = HashSet::<usize>::new();
    let mut groups = 0;
    for i in 0..graph.len() {
        if visited.contains(&i) { continue; }
        visited.insert(i);
        groups += 1;
        visit_from(i, graph, &mut visited);
    }
    return groups;
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|l| {
        let toks = l.split("<->").collect::<Vec<&str>>();
        toks[1].trim().split(",").map(|t| t.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>()
}

fn main() {
    let graph = parse_input(include_str!("input").trim());
    println!("{}", group_count(&graph));
}

#[test]
fn test_code() {
    let graph = parse_input(include_str!("input-simple").trim());
    let mut visited = HashSet::<usize>::new();
    visited.insert(0);
    visit_from(0, &graph, &mut visited);
    assert_eq!(visited.len(), 6);
}

#[test]
fn test_groups() {
    let graph = parse_input(include_str!("input-simple").trim());
    assert_eq!(group_count(&graph), 2);
}
