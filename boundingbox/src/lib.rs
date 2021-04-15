use std::{iter::Sum, ops::Add};

// Here is the implementation of a tree structure with AABB
pub mod quadtree;

pub type T = f64;

pub trait AABB {
    fn min(&self) -> Coord;
    fn max(&self) -> Coord;
    fn basic(&self) -> BasicAABB;
    fn middle(&self) -> Coord;
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: T,
    pub y: T,
}

impl Sum for Coord {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let m = iter.next().unwrap();
        iter.fold(m, |acc, x| Coord {
            x: acc.x + x.x,
            y: acc.y + x.y,
        })
    }
}

#[derive(Debug, Clone)]
pub struct BasicAABB(Coord, Coord);

impl BasicAABB {
    pub fn new(min: Coord, max: Coord) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        BasicAABB(min, max)
    }
}

impl AABB for BasicAABB {
    fn min(&self) -> Coord {
        self.0
    }

    fn max(&self) -> Coord {
        self.1
    }

    fn basic(&self) -> BasicAABB {
        self.clone()
    }

    fn middle(&self) -> Coord {
        let x: T = (self.0.x + self.1.x) / (2 as T);
        let y: T = (self.0.y + self.1.y) / (2 as T);

        Coord { x, y }
    }
}

impl Sum for BasicAABB {
    fn sum<T: Iterator<Item = Self>>(mut iter: T) -> Self {
        let first = iter.next().unwrap();

        iter.fold(first, |acc, x| acc + x)
    }
}

impl Add for BasicAABB {
    type Output = BasicAABB;

    fn add(self, rhs: Self) -> Self::Output {
        let a_min = self.min();
        let a_max = self.max();
        let b_min = rhs.min();
        let b_max = rhs.max();

        let n_min_x = a_min.x.min(b_min.x);
        let n_min_y = a_min.y.min(b_min.y);

        let n_max_x = a_max.x.max(b_max.x);
        let n_max_y = a_max.y.max(b_max.y);

        BasicAABB(
            Coord {
                x: n_min_x,
                y: n_min_y,
            },
            Coord {
                x: n_max_x,
                y: n_max_y,
            },
        )
    }
}
