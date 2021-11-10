use std::collections::VecDeque;

#[derive(Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const SPELLS: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

fn spell_cost(spell: &Spell) -> i32 {
    match spell {
        Spell::MagicMissile => 53,
        Spell::Drain => 73,
        Spell::Shield => 113,
        Spell::Poison => 173,
        Spell::Recharge => 229,
    }
}

#[derive(Debug, Clone)]
struct GameState {
    health: i32,
    armor: i32,
    mana: i32,
    mana_used: i32,
    boss_health: i32,
    boss_damage: i32,
    boss_poisoned: i32,
    shielded: i32,
    recharging: i32,
}

enum Outcome {
    Win,
    Lose,
    More(GameState),
}

impl GameState {
    fn new() -> GameState {
        GameState {
            health: 50,
            armor: 0,
            mana: 500,
            mana_used: 0,
            boss_health: 71,
            boss_damage: 10,
            boss_poisoned: 0,
            shielded: 0,
            recharging: 0,
        }
    }

    fn can_cast(&self, spell: &Spell) -> bool {
        if self.mana >= spell_cost(spell) {
            match spell {
                Spell::Shield => self.shielded <= 1,
                Spell::Recharge => self.recharging <= 1,
                Spell::Poison => self.boss_poisoned <= 1,
                _ => true,
            }
        } else {
            false
        }
    }

    // returns true if boss died
    fn check_effects(&mut self) -> bool {
        if self.shielded > 0 {
            self.shielded -= 1;
        }
        if self.recharging > 0 {
            self.mana += 101;
            self.recharging -= 1;
        }
        if self.boss_poisoned > 0 {
            self.boss_health -= 3;
            self.boss_poisoned -= 1;
            if self.boss_health <= 0 {
                return true;
            }
        }
        false
    }

    fn cast(&self, spell: &Spell, hard_mode: bool) -> Outcome {
        let mut new_state = self.clone();

        if hard_mode {
            new_state.health -= 1;
            if new_state.health <= 0 {
                return Outcome::Lose;
            }
        }

        // Check effects
        if new_state.check_effects() {
            return Outcome::Win;
        }

        // Cast spell
        new_state.mana -= spell_cost(spell);
        new_state.mana_used += spell_cost(spell);
        match spell {
            Spell::MagicMissile => new_state.boss_health -= 4,
            Spell::Drain => {
                new_state.boss_health -= 2;
                new_state.health += 2;
            }
            Spell::Shield => new_state.shielded += 6,
            Spell::Poison => new_state.boss_poisoned += 6,
            Spell::Recharge => new_state.recharging += 5,
        }
        if new_state.boss_health <= 0 {
            return Outcome::Win;
        }

        // Check effects
        if new_state.check_effects() {
            return Outcome::Win;
        }

        // Boss attacks
        new_state.health -= if new_state.shielded > 0 {
            std::cmp::max(self.boss_damage - 7, 1)
        } else {
            self.boss_damage
        };
        if new_state.health <= 0 {
            return Outcome::Lose;
        }

        // Next round
        Outcome::More(new_state)
    }
}

fn solve(hard_mode: bool) -> i32 {
    let mut queue = VecDeque::<GameState>::new();
    queue.push_back(GameState::new());
    while let Some(state) = queue.pop_front() {
        for spell in SPELLS.iter().filter(|s| state.can_cast(s)) {
            match state.cast(spell, hard_mode) {
                Outcome::Win => return state.mana_used + spell_cost(spell),
                Outcome::Lose => (),
                Outcome::More(new_state) => queue.push_back(new_state),
            }
        }
    }
    panic!("nooooo");
}

fn main() {
    println!("{}", solve(false));
    println!("{}", solve(true));
}
