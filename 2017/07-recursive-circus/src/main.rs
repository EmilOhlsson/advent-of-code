use std::collections::HashMap;

fn find_imbalance(map: &HashMap<String, Vec<String>>,
                  weights: &HashMap<String, usize>,
                  root: &String)
                  -> (usize, bool) {
    let mut sum = 0;
    let vec = map.get(root).unwrap();
    let mut ws = Vec::new();
    let mut child_balanced = true;
    for child in vec {
        let (tree_weight, balanced) = find_imbalance(map, weights, child);
        child_balanced = child_balanced && balanced;
        sum += tree_weight;
        ws.push(tree_weight);
    }
    ws.sort();
    ws.dedup();
    if ws.len() >= 2 && child_balanced {
        println!("Candidate: {:?}", ws);
        for child in vec {
            let (tree_weight, _) = find_imbalance(map, weights, child);
            println!("{} = {} + {}", tree_weight, weights.get(child).unwrap(), tree_weight - weights.get(child).unwrap());
        }
        child_balanced = false;
    }
    let gnome_weight = *weights.get(root).unwrap();
    return (sum + gnome_weight, child_balanced);
}

fn main() {
    let mut gnomes = HashMap::<String, usize>::new();
    let mut parents = HashMap::<String, String>::new();
    let mut tower = HashMap::<String, Vec<String>>::new();

    let input = include_str!("input");
    input.lines().for_each(|l| {
        let toks = l.split_whitespace().collect::<Vec<&str>>();
        let name = String::from(toks[0]);
        let weight = toks[1]
            .trim_matches(|p| p == '(' || p == ')')
            .parse::<usize>()
            .unwrap();
        tower.insert(name.clone(), Vec::new());
        if toks.len() > 2 {
            for i in 3..toks.len() {
                let child = String::from(toks[i].trim_matches(|c| c == ','));
                parents.insert(child.clone(), name.clone());
                if let Some(g) = tower.get_mut(&name) {
                    g.push(child.clone());
                }
            }
        }
        gnomes.insert(name.clone(), weight);
    });
    for (_, v) in &parents {
        let mut parent: String = v.clone();
        loop {
            if let Some(p) = parents.get(&parent) {
                parent = p.clone();
            } else {
                println!("{}", parent);
                find_imbalance(&tower, &gnomes, &parent);
                break;
            }
        }
        break;
    }
}
