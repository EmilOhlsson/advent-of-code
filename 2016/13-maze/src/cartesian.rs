#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Cartesian {
    pub x: i32,
    pub y: i32,
}

impl Cartesian {
    pub fn new(x: i32, y: i32) -> Cartesian {
        Cartesian { x: x, y: y }
    }

    /// Creates a list of points around `self` excluding diagonal
    pub fn neigh4(&self) -> Vec<Cartesian> {
        let x = self.x;
        let y = self.y;
        vec![
            Cartesian::new(x - 1, y),
            Cartesian::new(x, y + 1),
            Cartesian::new(x + 1, y),
            Cartesian::new(x, y - 1),
        ]
    }

    /// Creates a list of points around `self` including diagonal
    pub fn neigh8(&self) -> Vec<Cartesian> {
        let x = self.x;
        let y = self.y;
        vec![
            Cartesian::new(x - 1, y),
            Cartesian::new(x - 1, y + 1),
            Cartesian::new(x, y + 1),
            Cartesian::new(x + 1, y + 1),
            Cartesian::new(x + 1, y),
            Cartesian::new(x + 1, y - 1),
            Cartesian::new(x, y - 1),
            Cartesian::new(x - 1, y - 1),
        ]
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        let x_dist = (self.x - other.x).abs() as usize;
        let y_dist = (self.y - other.y).abs() as usize;
        x_dist + y_dist
    }
}
