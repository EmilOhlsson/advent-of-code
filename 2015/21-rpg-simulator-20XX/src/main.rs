#[derive(PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
}

#[derive(Debug)]
struct Character {
    hp: i32,
    dmg: i32,
    def: i32,
}

impl Character {
    // Let other character attack this charater, returns true if survived attack
    fn attack_by(&mut self, other: &Character) -> bool {
        self.hp -= std::cmp::max(1, other.dmg - self.def);
        self.hp > 0
    }
}

fn fight(def: i32, dmg: i32) -> Outcome {
    let mut boss = Character {
        hp: 109,
        dmg: 8,
        def: 2,
    };
    let mut player = Character { hp: 100, dmg, def };
    loop {
        if !boss.attack_by(&player) {
            return Outcome::Win;
        }
        if !player.attack_by(&boss) {
            return Outcome::Lose;
        }
    }
}

fn solve() -> (u32, u32) {
    let weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armors = [
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings = [
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];
    let mut minimum_winning_price = std::u32::MAX;
    let mut maximum_losing_price = std::u32::MIN;
    for armor in &armors {
        for weapon in &weapons {
            for (l, lring) in rings.iter().enumerate() {
                for (r, rring) in rings.iter().enumerate() {
                    if l == r {
                        continue;
                    }
                    if fight(armor.2 + rring.2 + lring.2, weapon.1 + rring.1 + lring.1)
                        == Outcome::Win
                    {
                        minimum_winning_price = std::cmp::min(
                            minimum_winning_price,
                            armor.0 + rring.0 + lring.0 + weapon.0,
                        );
                    } else {
                        maximum_losing_price = std::cmp::max(
                            maximum_losing_price,
                            armor.0 + rring.0 + lring.0 + weapon.0,
                        );
                    }
                }
            }
        }
    }
    (minimum_winning_price, maximum_losing_price)
}

fn main() {
    println!("{:?}", solve());
}
