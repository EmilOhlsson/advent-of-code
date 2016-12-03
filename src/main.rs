enum Turn {
    Left(usize),
    Right(usize),
}

enum Heading {
    North,
    West,
    South,
    East,
}

struct Path {
    x: isize,
    y: isize,
    heading: Heading,
}

impl Path {
    fn walk(&mut self, turn: Turn) {
    }

    fn new() -> Path {
        Path {
            x: 0,
            y: 0,
            heading: Heading::North
        }
    }
}

fn main() {
    println!("Hello, world!");

}
