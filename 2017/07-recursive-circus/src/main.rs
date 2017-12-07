use std::collections::HashMap;

fn main() {
    let mut gnomes = HashMap::<String, usize>::new();
    let mut tower = HashMap::<String, String>:: new();
    let input = include_str!("input");
    input.lines().for_each(|l| {
        let toks = l.split_whitespace().collect::<Vec<&str>>();
        let name = String::from(toks[0]);
        let weight = toks[1]
            .trim_matches(|p| p == '(' || p == ')')
            .parse::<usize>().unwrap();
        if toks.len() > 2 {
            for i in 3..toks.len() {
                tower.insert(String::from(toks[i].trim_matches(|c| c == ',')), name.clone());
            }
        }
        gnomes.insert(name.clone(), weight);
    });
    for (_, v) in &tower {
        let mut parent: String =  v.clone();
        loop {
            if let Some(p) = tower.get(&parent) {
                parent = p.clone();
            } else {
                println!("{}", parent);
                break;
            }
        }
        break;
    }
}
