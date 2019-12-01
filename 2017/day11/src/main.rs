mod input;

use input::get_path;
use input::Direction;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0, z: 0 }
    }
    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y, z: self.z }
    }

    fn _move<'a>(&'a mut self, direction: &Direction) -> &'a Self {
        match direction {
            &Direction::Ne => {self.x += 1; self.z -= 1;},
            &Direction::Se => {self.x += 1; self.y -= 1;},
            &Direction::S => {self.z += 1; self.y -= 1;},
            &Direction::Sw => {self.z += 1; self.x -= 1;},
            &Direction::Nw => {self.y += 1; self.x -= 1;},
            &Direction::N => {self.y += 1; self.z -= 1;},
        }
        self
    }

    fn max_abs(&self) -> i32 {
        self.x.abs().max(self.y.abs()).max(self.z.abs())
    }
}

fn main() {
    let path: Vec<Direction> = get_path();
    let mut pos = Point::new();
    let mut top = 0;

    path.iter().for_each(|d| {
        top = top.max(pos._move(&d).max_abs())
    });

    println!("{:?}", (pos.max_abs(), top));
}
