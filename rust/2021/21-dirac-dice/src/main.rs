use cached::proc_macro::cached;

fn roll_dice(dice: &mut usize, count: &mut usize) -> usize {
    let mut result = 0;
    for _ in 0..3 {
        *dice += 1;
        result += *dice;
    }
    *count += 3;
    result
}

fn solve_p1(mut player1: usize, mut player2: usize) -> usize {
    let mut score1 = 0;
    let mut score2 = 0;

    let mut dice = 0;
    let mut roll_count = 0;

    player1 -= 1;
    player2 -= 1;
    loop {
        player1 = (player1 + roll_dice(&mut dice, &mut roll_count)) % 10;
        score1 += player1 + 1;
        if score1 >= 1000 {
            return roll_count * score2;
        }

        player2 = (player2 + roll_dice(&mut dice, &mut roll_count)) % 10;
        score2 += player2 + 1;
        if score2 >= 1000 {
            return roll_count * score1;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PlayerState {
    position: usize,
    score: usize,
}

impl PlayerState {
    fn new(pos: usize) -> PlayerState {
        PlayerState {
            position: pos - 1,
            score: 0,
        }
    }

    fn mv(&self, steps: usize) -> PlayerState {
        let position = (self.position + steps) % 10;
        PlayerState {
            position,
            score: self.score + position + 1,
        }
    }

    fn won(&self) -> bool {
        self.score >= 21
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Player {
    One,
    Two,
}

impl Player {
    fn next(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

fn solve_v2(player1: usize, player2: usize) -> u64 {
    #[cached]
    fn quantum_play(turn: Player, one: PlayerState, two: PlayerState) -> (u64, u64) {
        let mut wins = (0, 0);
        for (steps, univ) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let (mut one, mut two) = (one, two);
            match turn {
                Player::One => one = one.mv(steps),
                Player::Two => two = two.mv(steps),
            }

            if one.won() {
                wins.0 += univ;
            } else if two.won() {
                wins.1 += univ;
            } else {
                let (a, b) = quantum_play(turn.next(), one, two);
                wins = (wins.0 + univ * a, wins.1 + univ * b);
            }
        }
        wins
    }
    let (a, b) = quantum_play(
        Player::One,
        PlayerState::new(player1),
        PlayerState::new(player2),
    );
    std::cmp::max(a, b)
}

fn main() {
    println!("{}", solve_p1(8, 2));
    println!("{}", solve_v2(8, 2));
}

#[test]
fn test_part1() {
    assert_eq!(solve_p1(4, 8), 739785);
}

#[test]
fn part2() {
    assert_eq!(solve_v2(4, 8), 444356092776315);
}
