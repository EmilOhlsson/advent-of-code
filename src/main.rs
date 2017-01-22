use std::fmt::Display;
use std::collections::VecDeque;

struct Node<T: Display> {
    data: T,
    childs: Vec<Node<T>>,
}

impl <T: Display> Node<T> {
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

/* State
 * (1-4, 1-4, 1-4, 1-4, 1-4)
 * (1, 2, 1, 3, 1) ->
 *      (2, 2, 2, 3, 2)
 *      (2, 2, 2, 3, 1)
 *      (2, 2, 1, 3, 2)
 */
fn moves(state: &[i32; 5]) -> Vec<[i32; 5]> {
    let mut mv = Vec::new();
    {
        let f_new = state[0] + 1;
        if f_new < 4 {
            for (i, f) in state.iter().enumerate() {
                println!("({},{})", i, f);
            }
        }
    }
    mv
}

fn main() {
    /* TODO: Build representation of state (preferably small)
     * and then build a tree of all the initial moves, and
     * then search down and remove sub-trees which are dead
     * ends */

    /* Possible implementation: Create an iterator that
     * iterates of the nodes in the tree? */
    println!("Hello, world!");

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

    moves(&[1, 2, 1, 3, 1]);
}
