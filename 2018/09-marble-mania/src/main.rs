use std::collections::VecDeque;

fn ccwise(marbles: &mut VecDeque<usize>, steps: usize) {
    for _ in 0..steps {
        if let Some(e) = marbles.pop_back() {
            marbles.push_front(e);
        }
    }
}

fn cwise(marbles: &mut VecDeque<usize>, steps: usize) {
    for _ in 0..steps {
        if let Some(e) = marbles.pop_front() {
            marbles.push_back(e);
        }
    }
}

fn solve(debug: bool, player_count: usize, last_marble_score: usize) -> usize {
    let mut players = vec![0; player_count];
    let mut marbles: VecDeque<usize> = VecDeque::new();
    marbles.insert(0, 0);

    for i in 1..=last_marble_score {
        if i % 23 == 0 {
            let player_score = players.get_mut(i % player_count).unwrap();
            ccwise(&mut marbles, 7);
            *player_score += i + marbles.pop_back().unwrap();
            cwise(&mut marbles, 1);
        } else {
            cwise(&mut marbles, 1);
            marbles.push_back(i);
        }

        if debug {
            println!("[{}] {:?}", (i - 1) % player_count + 1, marbles);
        }
    }

    *players.iter().max().unwrap()
}

fn main() {
    println!("{}", solve(false, 426, 72058));
    println!("{}", solve(false, 426, 72058 * 100));
}

#[test]
fn test() {
    assert_eq!(solve(true, 9, 25), 32);
    assert_eq!(solve(false, 10 ,1618), 8317);
    assert_eq!(solve(false, 13 ,7999), 146373);
    assert_eq!(solve(false, 17 ,1104), 2764);
    assert_eq!(solve(false, 21 ,6111), 54718);
    assert_eq!(solve(false, 30 ,5807), 37305);
}
