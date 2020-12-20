use regex::Regex;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

const TILE_WIDTH: usize = 10;
type Tile = Vec<Vec<u16>>;

fn reverse(v: u16) -> u16 {
    (0..TILE_WIDTH).fold(0, |a, n| (a << 1) | ((v >> n) & 1))
}

// TOP: 0
// LEFT: 1
// BOT: 2
// RIGHT: 3

/// Rotate counter clockwise
fn rotate(piece: [u16; 4]) -> [u16; 4] {
    [piece[3], reverse(piece[0]), piece[1], reverse(piece[2])]
}

fn flip(piece: [u16; 4]) -> [u16; 4] {
    [reverse(piece[0]), piece[3], reverse(piece[2]), piece[1]]
}

/// Rotation in steps counter clockwise, and if it was flipped before rotating
type Rotation = (u8, bool);

/// Generate a list of rotations and flips for a given tile. The list contains pairs of edge
/// patterns and rotations
fn get_placements(tile: &Tile) -> Vec<([u16; 4], Rotation)> {
    let mut piece = [0u16; 4];
    for i in 0..TILE_WIDTH {
        piece[0] = piece[0] << 1 | tile[0][i];
        piece[1] = piece[1] << 1 | tile[i][0];
        piece[2] = piece[2] << 1 | tile[9][i];
        piece[3] = piece[3] << 1 | tile[i][9];
    }

    let mut result = Vec::new();
    for rot in 0..4 {
        let piece_flipped = flip(piece);
        result.push((piece, (rot, false)));
        result.push((piece_flipped, (rot, true)));
        piece = rotate(piece);
    }
    result
}

fn isqrt(n: i32) -> Option<i32> {
    (1..=20).find(|i| i * i == n)
}

fn get_tiles(input: &str) -> HashMap<u32, Tile> {
    let re = Regex::new(r"^Tile (\d+):$").unwrap();
    let mut tile = Vec::new();
    let mut tile_id = 0;
    let mut tiles = HashMap::new();
    for line in input.lines() {
        if let Some(tile_cap) = re.captures(line) {
            tile_id = tile_cap[1].parse::<u32>().unwrap();
            tile = Vec::new();
        } else if line == "" {
            tiles.insert(tile_id, tile.clone());
        } else {
            tile.push(line.chars().map(|ch| (ch == '#') as u16).collect());
        }
    }
    tiles.insert(tile_id, tile);
    tiles
}

// Tile id, bottom pattern, right pattern, rotation
type Placement = (u32, u16, u16, Rotation);

/// State describing a puzzle solving step
struct PuzzleState<'a> {
    next: i32,
    width: i32,

    /// Mapping from left side to (id, bottom, right)
    tops: &'a HashMap<u16, Vec<Placement>>,

    /// Mapping from top side to (id, bottom, right)
    lefts: &'a HashMap<u16, Vec<Placement>>,

    placed: HashSet<u32>,
    placements: HashMap<(i32, i32), Placement>, // position -> (id, bottom, right)
    tile_rotations: &'a HashMap<u32, Vec<([u16; 4], Rotation)>>,
    tiles: &'a HashMap<u32, Tile>,
}

impl<'a> PuzzleState<'a> {
    fn try_place(&self) -> Vec<PuzzleState<'a>> {
        let (row, col) = (self.next / self.width, self.next % self.width);
        let left = self.placements.get(&(row, col - 1));
        let top = self.placements.get(&(row - 1, col));
        let mut candidates = HashSet::<Placement>::new();

        // Add all eligable candidates for the next candidate. For the initial state this is all
        // states. For other states is all non placed tiles, with matching sides
        if left.is_some() && top.is_some() {
            let t = top.unwrap();
            let l = left.unwrap();

            if let Some((top_match, left_match)) = self.tops.get(&t.1).zip(self.lefts.get(&l.2)) {
                for tm in top_match {
                    for lm in left_match {
                        if tm == lm && !self.placed.contains(&lm.0) {
                            candidates.insert(*lm);
                        }
                    }
                }
            }
        } else if left.is_none() && top.is_some() {
            let t = top.unwrap();
            if let Some(ms) = self.tops.get(&t.1) {
                for (id, bot, right, rotation) in ms {
                    if !self.placed.contains(id) {
                        candidates.insert((*id, *bot, *right, *rotation));
                    }
                }
            }
        } else if left.is_some() && top.is_none() {
            let l = left.unwrap();
            if let Some(ms) = self.lefts.get(&l.2) {
                for (id, bot, right, rotation) in ms {
                    if !self.placed.contains(id) {
                        candidates.insert((*id, *bot, *right, *rotation));
                    }
                }
            }
        } else {
            // Only initial state:
            for (tile, rotations) in self.tile_rotations {
                for (edges, rotation) in rotations {
                    candidates.insert((*tile, edges[2], edges[3], *rotation));
                }
            }
        }

        let mut result = Vec::new();
        for candidate in &candidates {
            let mut placements = self.placements.clone();
            let mut placed = self.placed.clone();
            placed.insert(candidate.0);
            placements.insert((row, col), *candidate);
            result.push(PuzzleState {
                next: self.next + 1,
                width: self.width,
                tops: self.tops,
                lefts: self.lefts,
                placements,
                placed,
                tile_rotations: self.tile_rotations,
                tiles: self.tiles,
            })
        }
        result
    }
}

fn translate(
    (row, col): (i32, i32),
    (rows, cols): (i32, i32),
    (rotation, flipped): Rotation,
) -> (i32, i32) {
    match rotation {
        0 => (row, if flipped { cols - col - 1 } else { col }),
        1 => (if flipped { cols - col - 1 } else { col }, rows - row - 1),
        2 => (rows - row - 1, if flipped { col } else { cols - col - 1 }),
        3 => (if flipped { col } else { cols - col - 1 }, row),
        _ => panic!("Invalid rotation"),
    }
}

// This is part 2
fn find_monsters(puzzle: &PuzzleState) -> u64 {
    let mut map = HashSet::<(i32, i32)>::new();
    let mut monster = HashSet::<(i32, i32)>::new();
    let monster_str = include_str!("monster");
    for (row, line) in monster_str.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                monster.insert((row as i32, col as i32));
            }
        }
    }

    // Build map by placing rotated and flipped tiles (without borders) next to each other
    let step = TILE_WIDTH as i32 - 2;
    let mut map_size = (0i32, 0i32);
    let tile_size = (TILE_WIDTH as i32, TILE_WIDTH as i32);
    for row_puzzle in 0..puzzle.width {
        for col_puzzle in 0..puzzle.width {
            let (tile_id, _, _, tile_rotation) =
                puzzle.placements.get(&(row_puzzle, col_puzzle)).unwrap();
            let tile = puzzle.tiles.get(tile_id).unwrap();

            for tile_row in 0..step {
                for tile_col in 0..step {
                    let (r, c) = translate((tile_row + 1, tile_col + 1), tile_size, *tile_rotation);
                    let (row_dst, col_dst) =
                        (row_puzzle * step + tile_row, col_puzzle * step + tile_col);

                    if tile[r as usize][c as usize] != 0 {
                        map.insert((row_dst, col_dst));
                    }
                    map_size = (max(map_size.0, row_dst + 1), max(map_size.1, col_dst + 1));
                }
            }
        }
    }

    // Build a stencil of monsters, one monster starting for each point in the map grid
    let mut monster_stencils = Vec::new();
    for row in 0..map_size.0 {
        for col in 0..map_size.1 {
            monster_stencils.push(
                monster
                    .iter()
                    .map(|(r, c)| (row + r, col + c))
                    .collect::<HashSet<(i32, i32)>>(),
            );
        }
    }

    // Generate list of all kind of rotations and flips of the map
    let mut map_rotations = Vec::new();
    for rotation in 0..4 {
        for flip in &[false, true] {
            map_rotations.push(
                map.iter()
                    .map(|rc| translate(*rc, map_size, (rotation, *flip)))
                    .collect::<HashSet<(i32, i32)>>(),
            );
        }
    }

    // Search for map rotations with monsters in them
    for map in &map_rotations {
        let mut monsters_found = 0;

        for stencil in &monster_stencils {
            for map in &map_rotations {
                monsters_found += map.is_superset(&stencil) as usize;
            }
        }

        if monsters_found > 0 {
            return (map.len() - monsters_found * monster.len()) as u64;
        }
    }
    panic!("Did not find monsters");
}

fn solve(input: &str) -> (u64, u64) {
    let tiles = get_tiles(input);

    // Map left/top side to an array of matching pieces, with their ID and opposite side
    let mut left_map = HashMap::<u16, Vec<Placement>>::new();
    let mut top_map = HashMap::<u16, Vec<Placement>>::new();
    let mut tile_map = HashMap::<u32, Vec<([u16; 4], Rotation)>>::new();
    for (id, tile) in &tiles {
        for (edges, rotation) in get_placements(tile) {
            let tile = tile_map.entry(*id).or_insert_with(Vec::new);
            let top = top_map.entry(edges[0]).or_insert_with(Vec::new);
            let left = left_map.entry(edges[1]).or_insert_with(Vec::new);
            top.push((*id, edges[2], edges[3], rotation));
            left.push((*id, edges[2], edges[3], rotation));
            tile.push((edges, rotation));
        }
    }

    // Push initial empty puzzle
    let width = isqrt(tiles.len() as i32).unwrap();
    let mut heap = VecDeque::<PuzzleState>::new();
    heap.push_back(PuzzleState {
        next: 0,
        width,
        tops: &top_map,
        lefts: &left_map,
        placed: HashSet::new(),
        placements: HashMap::new(),
        tile_rotations: &tile_map,
        tiles: &tiles,
    });

    // Do BFS search for puzzle solution
    while let Some(state) = heap.pop_front() {
        for state_new in state.try_place() {
            if state_new.next == width * width {
                let tl = state_new.placements[&(0, 0)].0 as u64;
                let tr = state_new.placements[&(0, width - 1)].0 as u64;
                let bl = state_new.placements[&(width - 1, 0)].0 as u64;
                let br = state_new.placements[&(width - 1, width - 1)].0 as u64;
                return (tl * tr * bl * br, find_monsters(&state_new));
            } else {
                heap.push_back(state_new);
            }
        }
    }
    panic!("was not able to solve puzzle");
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), (20899048083289, 273));
}
