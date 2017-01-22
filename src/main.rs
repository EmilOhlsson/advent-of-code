use std::fmt::Display;
use std::collections::VecDeque;

struct Node<T: Display> {
    data: T,
    childs: Vec<Node<T>>,
}

impl<T: Display> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            childs: Vec::new(),
        }
    }

    fn add(&mut self, data: T) {
        self.childs.push(Node::new(data));
    }

    fn attach(&mut self, node: Node<T>) {
        self.childs.push(node);
    }

    /**
     * Currently prints the tree breadth first
     */
    fn game_tree_search(&self) {
        let mut queue: VecDeque<&Node<T>> = VecDeque::new();
        queue.push_back(&self);

        while !queue.is_empty() {
            let n = queue.pop_front().unwrap();
            n.childs.iter().map(|n| queue.push_back(&n)).last();
            println!("node: {}", n.data);
        }
    }
}

// State
// (1-4, 1-4, 1-4, 1-4, 1-4)
// (1, 2, 1, 3, 1) ->
//      (2, 2, 2, 3, 2) (0, 1, 0, 1)
//      (2, 2, 2, 3, 1) (0, 1, 0, 0)
//      (2, 2, 1, 3, 2) (0, 0, 0, 1)

struct State {
    elev: isize,
    floors: [isize; 4],
}

type Move = [isize; 4];
const FLOORS: isize = 4;
fn moves(state: &State) -> Vec<Move> {
    fn valid_move(state: &State, filled: isize, mv: &Move) -> bool {
        if filled <= 0 || filled >= 2 {
            return false;
        }
        true
    }
    fn add_moves(state: &State,
           dir: isize,
           offs: usize,
           filled: isize,
           moves: &mut Vec<Move>,
           buffer: &mut Move) {
        if filled >= 2 {
            return;
        }
        for i in (offs)..(buffer.len()) {
            if state.floors[i] == state.elev {
                buffer[i] = dir;
                if valid_move(state, filled, &buffer) {
                    moves.push(buffer.clone());
                }
                add_moves(state, dir, i + 1, filled + 1, moves, buffer);
                buffer[i] = 0;
                add_moves(state, dir, i + 1, filled, moves, buffer);
            }
        }
    }

    let mut mv = Vec::new();

    if state.elev + 1 <= FLOORS {
        let mut buffer: Move = [0; 4];
        add_moves(state, 1, 0, 0, &mut mv, &mut buffer);
    }

    if state.elev - 1 >= 1 {
        let mut buffer: Move = [0; 4];
        add_moves(state, -1, 0, 0, &mut mv, &mut buffer);
    }

    mv
}

fn main() {
    // TODO: Build representation of state (preferably small)
    // and then build a tree of all the initial moves, and
    // then search down and remove sub-trees which are dead
    // ends

    // Possible implementation: Create an iterator that
    // iterates of the nodes in the tree?

    let mut root = Node::new(1);
    let mut ch_left = Node::new(2);
    let mut ch_right = Node::new(3);
    ch_left.add(4);
    ch_left.add(5);
    ch_right.add(6);
    ch_right.add(7);
    ch_right.add(8);
    root.attach(ch_left);
    root.attach(ch_right);

    root.game_tree_search();

    let state = State {
        elev: 1,
        floors: [2, 1, 3 ,1],
    };
    let mvs = moves(&state);
    for m in mvs {
        println!("{:?}", m);
    }
}
