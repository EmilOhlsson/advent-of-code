use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap};

trait Searchable {
    fn estimate_cost(&self) -> i32;
    fn next(&self) -> Vec<(i32, Box<Self>)>;
}

#[derive(PartialEq, Eq)]
struct ScoredNode<T>
where
    T: Eq + Searchable,
{
    score: i32,
    n: T,
}

impl<T> Ord for ScoredNode<T>
where
    T: Eq + Searchable,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl<T> PartialOrd for ScoredNode<T>
where
    T: Eq + Searchable,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> ScoredNode<T>
where
    T: Eq + Searchable,
{
    fn new(score: i32, n: T) -> ScoredNode<T> {
        ScoredNode { score, n }
    }
    fn distance(&self) -> i32 {
        self.n.estimate_cost()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Pod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Pod {
    fn from_char(ch: &char) -> Option<Pod> {
        match ch {
            'A' => Some(Pod::Amber),
            'B' => Some(Pod::Bronze),
            'C' => Some(Pod::Copper),
            'D' => Some(Pod::Desert),
            _ => None,
        }
    }

    fn dest(&self) -> usize {
        match self {
            Pod::Amber => 0,
            Pod::Bronze => 1,
            Pod::Copper => 2,
            Pod::Desert => 3,
        }
    }

    fn movement_cost(&self) -> i32 {
        match self {
            Pod::Amber => 1,
            Pod::Bronze => 10,
            Pod::Copper => 100,
            Pod::Desert => 1000,
        }
    }
}

type Burrow<const N: usize> = [Option<Pod>; N];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cave<const N: usize> {
    hallway: [Option<Pod>; 11],
    burrows: [Burrow<N>; 4],
}

fn create_burrow<const N: usize>(burrow: &[char; N]) -> [Option<Pod>; N] {
    let mut result = [None; N];
    result
        .iter_mut()
        .zip(burrow.iter())
        .for_each(|(dst, src)| *dst = Pod::from_char(src));
    result
}

impl<const N: usize> Cave<N> {
    fn new(a: &[char; N], b: &[char; N], c: &[char; N], d: &[char; N]) -> Cave<N> {
        Cave {
            hallway: Default::default(),
            burrows: [
                create_burrow(a),
                create_burrow(b),
                create_burrow(c),
                create_burrow(d),
            ],
        }
    }
}

impl<const N: usize> Searchable for Cave<N> {
    fn estimate_cost(&self) -> i32 {
        let mut dist = 0;

        for (src, burrow) in self.burrows.iter().enumerate() {
            for pod in burrow.iter().filter_map(|p| *p) {
                let dst = pod.dest();
                dist += 2 * (dst as i32 - src as i32).abs() * pod.movement_cost();
            }
        }

        for (src, pod) in self.hallway.iter().enumerate() {
            if let Some(pod) = pod {
                let dst = 2 + 2 * pod.dest();
                dist += (dst as i32 - src as i32).abs() * pod.movement_cost();
            }
        }
        dist
    }

    fn next(&self) -> Vec<(i32, Box<Self>)> {
        let mut result = Vec::new();
        let valid = [0, 1, 3, 5, 7, 9, 10];

        let diff = |a: usize, b: usize| -> usize { max(a, b) - min(a, b) };
        let burrow_to_hallway = |i: usize| -> usize { 2 * (1 + i) };

        let passable = |src: usize, dst: usize| -> bool {
            assert!(src != dst);
            let (lo, hi) = (min(src, dst), max(src, dst));
            self.hallway[lo..=hi].iter().all(Option::is_none)
        };

        /* It's only allowed to move to a burrow that only
         * contains the proper kind of Amphipods */
        let allowed_burrow = |b: usize| -> bool {
            self.burrows[b]
                .iter()
                .all(|slot| slot.map(|pod| pod.dest() == b).unwrap_or(true))
        };

        let passable_from_hallway = |src: usize, dst: usize| -> bool {
            assert!(src != dst);
            if src < dst {
                self.hallway[(src + 1)..=dst].iter().all(Option::is_none)
            } else {
                self.hallway[dst..src].iter().all(Option::is_none)
            }
        };

        let top_of_burrow = |b: usize| -> (usize, &Option<Pod>) {
            self.burrows[b]
                .iter()
                .enumerate()
                .find(|(_, p)| p.is_some())
                .unwrap_or((N, &None))
        };

        let move_possible = |src: usize| -> bool {
            self.burrows[src]
                .iter()
                .any(|p| p.map(|p| p.dest() != src).unwrap_or(false))
        };

        /* Check for valid movements starting in burrows */
        for src_burrow in 0..self.burrows.len() {
            let src = burrow_to_hallway(src_burrow);
            if move_possible(src_burrow) {
                let (src_depth, src_pod) = top_of_burrow(src_burrow);
                if let Some(src_pod) = src_pod {
                    let dst_burrow = src_pod.dest();
                    let dst = burrow_to_hallway(dst_burrow);
                    let (dst_depth, _) = top_of_burrow(dst_burrow);
                    if src_burrow != dst_burrow
                        && dst_depth > 0
                        && passable(src, dst)
                        && allowed_burrow(dst_burrow)
                    {
                        /* Move from burrow to another burrow */
                        let mut next_state = *self;
                        next_state.burrows[src_burrow][src_depth] = None;
                        next_state.burrows[dst_burrow][dst_depth - 1] = Some(*src_pod);
                        let distance = src_depth + 1 + dst_depth + diff(src, dst);
                        result.push((
                            src_pod.movement_cost() * distance as i32,
                            Box::new(next_state),
                        ));
                    }

                    for dst in valid
                        .iter()
                        .filter(|dst| src != **dst && passable(src, **dst))
                    {
                        /* Move to a valid position in hallway */
                        let mut next_state = *self;
                        next_state.burrows[src_burrow][src_depth] = None;
                        next_state.hallway[*dst] = Some(*src_pod);
                        let distance = src_depth + 1 + diff(src, *dst);
                        result.push((
                            src_pod.movement_cost() * distance as i32,
                            Box::new(next_state),
                        ));
                    }
                }
            }
        }

        /* Check for valid movements starting in hallway */
        for (src, src_pod) in self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(i, h)| h.map(|p| (i, p)))
        {
            let dst_burrow = src_pod.dest();
            let dst = burrow_to_hallway(dst_burrow);
            let (dst_depth, _) = top_of_burrow(dst_burrow);

            /* It's OK to move in corridor to a burrow only if there is a way there
             * and if target burrow only contains the same Amphipods */
            if dst_depth > 0 && passable_from_hallway(src, dst) && allowed_burrow(dst_burrow) {
                /* Move from hallway to burrow */
                let mut next_state = *self;
                next_state.hallway[src] = None;
                next_state.burrows[dst_burrow][dst_depth - 1] = Some(src_pod);
                let distance = dst_depth + diff(src, dst);
                result.push((
                    src_pod.movement_cost() * distance as i32,
                    Box::new(next_state),
                ));
            }
        }

        result
    }
}

fn search<const N: usize>(start: &Cave<N>, print_steps: bool) -> i32 {
    let mut queue = BinaryHeap::<ScoredNode<Cave<N>>>::new();
    let mut total_cost_at = HashMap::<Cave<N>, i32>::new();

    total_cost_at.insert(*start, 0);
    queue.push(ScoredNode::new(0, *start));

    /* This is not really needed, but nice to be able to reconstruct */
    let mut came_from = HashMap::<Cave<N>, Cave<N>>::new();
    while let Some(current) = queue.pop() {
        if current.distance() == 0 {
            /* Done, print and return result */
            if print_steps {
                let mut cave = current.n;
                println!("Cost: {:?}", total_cost_at.get(&cave));
                print_cave(&cave);
                while let Some(prev) = came_from.get(&cave) {
                    cave = *prev;
                    println!("Cost: {:?}", total_cost_at.get(&cave));
                    print_cave(&cave);
                }
            }
            return total_cost_at[&current.n];
        }

        let cost_to_current = total_cost_at[&current.n];
        for (cost_next, next) in current.n.next() {
            let new_cost = cost_to_current + cost_next;
            let prev_cost = total_cost_at.entry(*next).or_insert(i32::MAX);

            /* Found a cheaper way, */
            if new_cost < *prev_cost {
                came_from.insert(*next, current.n);
                *prev_cost = new_cost;
                queue.push(ScoredNode::new(new_cost + next.estimate_cost(), *next));
            }
        }
    }
    panic!("Did not find any path");
}

fn print_cave<const N: usize>(burrows: &Cave<N>) {
    let to_char = |pod: Pod| -> char {
        match pod {
            Pod::Amber => 'A',
            Pod::Bronze => 'B',
            Pod::Copper => 'C',
            Pod::Desert => 'D',
        }
    };

    println!("##############");
    print!("#");
    for e in burrows.hallway {
        print!("{}", e.map(to_char).unwrap_or('.'));
    }
    println!("#");
    println!(
        "###{}#{}#{}#{}###",
        burrows.burrows[0][0].map(to_char).unwrap_or('.'),
        burrows.burrows[1][0].map(to_char).unwrap_or('.'),
        burrows.burrows[2][0].map(to_char).unwrap_or('.'),
        burrows.burrows[3][0].map(to_char).unwrap_or('.'),
    );
    println!(
        "  #{}#{}#{}#{}#  ",
        burrows.burrows[0][1].map(to_char).unwrap_or('.'),
        burrows.burrows[1][1].map(to_char).unwrap_or('.'),
        burrows.burrows[2][1].map(to_char).unwrap_or('.'),
        burrows.burrows[3][1].map(to_char).unwrap_or('.'),
    );
    if N == 4 {
        println!(
            "  #{}#{}#{}#{}#  ",
            burrows.burrows[0][2].map(to_char).unwrap_or('.'),
            burrows.burrows[1][2].map(to_char).unwrap_or('.'),
            burrows.burrows[2][2].map(to_char).unwrap_or('.'),
            burrows.burrows[3][2].map(to_char).unwrap_or('.'),
        );
        println!(
            "  #{}#{}#{}#{}#  ",
            burrows.burrows[0][3].map(to_char).unwrap_or('.'),
            burrows.burrows[1][3].map(to_char).unwrap_or('.'),
            burrows.burrows[2][3].map(to_char).unwrap_or('.'),
            burrows.burrows[3][3].map(to_char).unwrap_or('.'),
        );
    }
    println!("  #########  ");
    println!();
}

fn main() {
    let input = Cave::<2>::new(&['D', 'C'], &['A', 'C'], &['A', 'B'], &['D', 'B']);
    println!("{}", search(&input, false));

    let input = Cave::<4>::new(
        &['D', 'D', 'D', 'C'],
        &['A', 'C', 'B', 'C'],
        &['A', 'B', 'A', 'B'],
        &['D', 'A', 'C', 'B'],
    );
    println!("{}", search(&input, false));
}

#[test]
fn simple_p1() {
    let input = Cave::new(&['B', 'A'], &['C', 'D'], &['B', 'C'], &['D', 'A']);
    assert_eq!(search(&input, true), 12521);
}

#[test]
fn simple_p2() {
    let input = Cave::new(
        &['B', 'D', 'D', 'A'],
        &['C', 'C', 'B', 'D'],
        &['B', 'B', 'A', 'C'],
        &['D', 'A', 'C', 'A'],
    );
    assert_eq!(search(&input, true), 44169);
}

#[test]
fn distance() {
    let goal = Cave::new(&['A', 'A'], &['B', 'B'], &['C', 'C'], &['D', 'D']);
    assert_eq!(goal.estimate_cost(), 0);
}
