use std::collections::HashMap;

fn main() {
    let mut gnomes = HashMap::<String, usize>::new();
    let mut parents = HashMap::<String, String>:: new();
    let mut tower = HashMap::<String, Vec<String>>::new();

    let input = include_str!("input-simple");
    input.lines().for_each(|l| {
        let toks = l.split_whitespace().collect::<Vec<&str>>();
        let name = String::from(toks[0]);
        let weight = toks[1]
            .trim_matches(|p| p == '(' || p == ')')
            .parse::<usize>().unwrap();
        tower.insert(name.clone(), Vec::new());
        if toks.len() > 2 {
            for i in 3..toks.len() {
                let child = String::from(toks[i].trim_matches(|c| c == ','));
                parents.insert(child.clone(), name.clone());
                //tower.get_mut(&name).unwrap().append(&mut child.clone());
            }
        }
        gnomes.insert(name.clone(), weight);
    });
    for (_, v) in &parents {
        let mut parent: String =  v.clone();
        loop {
            if let Some(p) = parents.get(&parent) {
                parent = p.clone();
            } else {
                println!("{}", parent);
                break;
            }
        }
        break;
    }
}
