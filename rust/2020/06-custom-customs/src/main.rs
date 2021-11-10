struct Group {
    yess: std::collections::HashMap<char, u32>,
    size: u32,
}

impl Group {
    fn end(&mut self) -> (usize, usize) {
        let mut p1 = 0;
        let mut p2 = 0;
        for count in self.yess.values() {
            p1 += 1;
            p2 += (*count == self.size) as usize;
        }
        self.yess.clear();
        self.size = 0;
        (p1, p2)
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut group = Group {
        yess: std::collections::HashMap::new(),
        size: 0,
    };

    let mut answer = (0, 0);
    for line in input.lines() {
        if line == "" {
            let grp = group.end();
            answer = (answer.0 + grp.0, answer.1 + grp.1);
        } else {
            group.size += 1;
            for ch in line.chars() {
                let entry = group.yess.entry(ch).or_insert(0);
                *entry += 1;
            }
        }
    }
    let grp = group.end();
    answer = (answer.0 + grp.0, answer.1 + grp.1);

    answer
}
fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}
