use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum DamageType {
    Slashing,
    Bludgeoning,
    Radiation,
    Cold,
    Fire,
}

impl FromStr for DamageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "slashing" => Ok(DamageType::Slashing),
            "bludgeoning" => Ok(DamageType::Bludgeoning),
            "radiation" => Ok(DamageType::Radiation),
            "cold" => Ok(DamageType::Cold),
            "fire" => Ok(DamageType::Fire),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
enum Team {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone)]
struct Group {
    id: usize,
    team: Team,
    units: i32,
    hitpoints: i32,
    damage: i32,
    initiative: i32,
    damage_type: DamageType,
    weaknesses: Vec<DamageType>,
    immunities: Vec<DamageType>,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.units * self.damage
    }

    fn damage_dealt(&self, (damage_type, damage): (DamageType, i32)) -> i32 {
        if self.weaknesses.contains(&damage_type) {
            damage * 2
        } else if self.immunities.contains(&damage_type) {
            0
        } else {
            damage
        }
    }

    fn get_damage(&self) -> (DamageType, i32) {
        (self.damage_type, self.effective_power())
    }

    fn damage(&mut self, dmg: (DamageType, i32)) {
        let damage = self.damage_dealt(dmg);
        let units_killed = damage / self.hitpoints;
        self.units -= units_killed;
    }
}

#[derive(Debug)]
enum Victory {
    None,
    Draw,
    ImmuneSystem(i32),
    Infection(i32),
}

fn fight(groups: &mut HashMap<usize, Group>) -> Victory {
    let units_before_fight = groups.values().map(|g| g.units).sum::<i32>();
    let (immune_system, infection): (Vec<usize>, Vec<usize>) = groups
        .keys()
        .partition(|g| groups[&g].team == Team::ImmuneSystem);
    let mut targeted: HashSet<usize> = HashSet::new();
    let mut targets: HashMap<usize, usize> = HashMap::new();

    if immune_system.len() == 0 {
        return Victory::Infection(infection.iter().map(|id| groups[id].units).sum());
    } else if infection.len() == 0 {
        return Victory::ImmuneSystem(immune_system.iter().map(|id| groups[id].units).sum());
    }

    // Create a list of of targeters, sort by maximum dealable damage
    let mut targeteers = groups.keys().cloned().collect::<Vec<usize>>();
    targeteers.sort_by_key(|id| (-groups[id].effective_power(), -groups[id].initiative));

    let mut attackers = groups.keys().cloned().collect::<Vec<usize>>();
    attackers.sort_by_key(|id| -groups[id].initiative);

    // Pick targets
    for group_id in &targeteers {
        let target_group = match groups[&group_id].team {
            Team::ImmuneSystem => &infection,
            Team::Infection => &immune_system,
        };

        // create sorted list of untargeted
        let damage = groups[&group_id].get_damage();
        let candidates = target_group
            .iter()
            .filter(|id| !targeted.contains(id) && groups[id].damage_dealt(damage) > 0)
            .cloned()
            .collect::<Vec<_>>();

        if let Some(target) = candidates.iter().max_by_key(|id| {
            (
                groups[id].damage_dealt(damage),
                groups[id].effective_power(),
                groups[id].initiative,
            )
        }) {
            //println!(" -- {} => {:?} => {}", group_id, candidates, target);
            targeted.insert(*target);
            targets.insert(*group_id, *target);
        }
    }

    // Attack
    for attacker_id in attackers {
        if groups[&attacker_id].units <= 0 {
            // Group is dead, skip it
            continue;
        }

        let damage = groups[&attacker_id].get_damage();
        if let Some(target_id) = targets.get(&attacker_id) {
            if let Some(target) = groups.get_mut(&target_id) {
                target.damage(damage);
            }
        }
    }

    // Discard all dead groups
    groups.retain(|_, g| g.units > 0);
    let units_after_fight = groups.values().map(|g| g.units).sum::<i32>();
    if units_after_fight == units_before_fight {
        Victory::Draw
    } else {
        Victory::None
    }
}

fn solve(input: &str, search: bool) -> Victory {
    let mut groups: HashMap<usize, Group> = HashMap::new();
    let mut current_team = Team::ImmuneSystem;
    let mut id = 0;
    let re = Regex::new(r"(\d+) units each with (\d+) hit points( \(.*\))? with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)").unwrap();

    for line in input.lines() {
        let mut immunities = Vec::new();
        let mut weaknesses = Vec::new();
        if let Some(cap) = re.captures(line) {
            if let Some(vuln) = cap.get(3) {
                for props in vuln
                    .as_str()
                    .trim()
                    .trim_matches(|ch| ch == '(' || ch == ')')
                    .split(';')
                    .map(|s| s.trim())
                {
                    let toks = props
                        .split_whitespace()
                        .map(|t| t.trim_matches(|ch| ch == ','))
                        .collect::<Vec<_>>();
                    if toks[0] == "weak" {
                        toks.iter()
                            .skip(2)
                            .for_each(|t| weaknesses.push(t.parse::<DamageType>().unwrap()));
                    } else {
                        toks.iter()
                            .skip(2)
                            .for_each(|t| immunities.push(t.parse::<DamageType>().unwrap()));
                    }
                }
            }
            let group = Group {
                id,
                team: current_team,
                units: cap[1].parse::<i32>().unwrap(),
                hitpoints: cap[2].parse::<i32>().unwrap(),
                damage: cap[4].parse::<i32>().unwrap(),
                damage_type: cap[5].parse::<DamageType>().unwrap(),
                initiative: cap[6].parse::<i32>().unwrap(),
                immunities,
                weaknesses,
            };
            groups.insert(id, group);
            id += 1;
        } else {
            if line == "Infection:" {
                current_team = Team::Infection;
            }
        }
    }

    for boost in 0.. {
        let mut boosted_groups = groups.clone();
        for (_, group) in &mut boosted_groups {
            if group.team == Team::ImmuneSystem {
                group.damage += boost;
            }
        }
        loop {
            match fight(&mut boosted_groups) {
                Victory::None => (),
                Victory::ImmuneSystem(res) => {
                    return Victory::ImmuneSystem(res);
                }
                Victory::Infection(res) => {
                    if !search {
                        return Victory::Infection(res);
                    } else {
                        break;
                    }
                }
                Victory::Draw => break,
            }
        }
    }
    panic!("Broken");
}

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input, false));
    println!("{:?}", solve(input, true));
}

#[test]
fn test() {
    let input = include_str!("input-simple.txt");
    assert_eq!(solve(input), 5216);
}
