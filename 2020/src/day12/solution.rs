use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Iterator for Direction {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        match self {
            Direction::North => Some(Direction::East),
            Direction::East => Some(Direction::South),
            Direction::South => Some(Direction::West),
            Direction::West => Some(Direction::North),
        }
    }
}
impl DoubleEndedIterator for Direction {
    fn next_back(&mut self) -> Option<Self> {
        match self {
            Direction::North => Some(Direction::West),
            Direction::West => Some(Direction::South),
            Direction::South => Some(Direction::East),
            Direction::East => Some(Direction::North),
        }
    }
}

enum Rotation {
    Left,
    Right,
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new() -> Self {
        Waypoint { x: 10, y: 1 }
    }

    fn change_direction(&mut self, rotation: Rotation, amount: u32) {
        for _ in 0..amount / 90 {
            self.x ^= self.y;
            self.y ^= self.x;
            self.x ^= self.y;
            match rotation {
                Rotation::Left => self.x *= -1,
                Rotation::Right => self.y *= -1,
            }
        }
    }
}

struct Ship {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Ship {
            x: 0,
            y: 0,
            direction: Direction::East,
        }
    }

    fn change_direction(&mut self, rotation: Rotation, amount: u32) {
        match rotation {
            Rotation::Left => {
                for _ in 0..amount / 90 {
                    self.direction = self.direction.into_iter().rev().nth(0).unwrap();
                }
            }
            Rotation::Right => {
                for _ in 0..amount / 90 {
                    self.direction = self.direction.into_iter().nth(0).unwrap();
                }
            }
        }
    }

    fn forward(&mut self, amount: u32) {
        match self.direction {
            Direction::North => {
                self.y += amount as i32;
            }
            Direction::South => {
                self.y -= amount as i32;
            }
            Direction::East => {
                self.x += amount as i32;
            }
            Direction::West => {
                self.x -= amount as i32;
            }
        };
    }

    fn to_waypoint(&mut self, waypoint: &Waypoint, amount: u32) {
        self.y += waypoint.y * amount as i32;
        self.x += waypoint.x * amount as i32;
    }
}

pub fn find_manhattan_dist(file: PathBuf) {
    let string: String = read_to_string(file).unwrap();
    let lines: Vec<&str> = string.lines().collect();

    let mut ship = Ship::new();
    for line in lines {
        let amount = line[1..].parse::<i32>().unwrap();
        match line.chars().nth(0).unwrap() {
            'N' => {
                ship.y += amount;
            }
            'S' => {
                ship.y -= amount;
            }
            'E' => {
                ship.x += amount;
            }
            'W' => {
                ship.x -= amount;
            }
            'L' => ship.change_direction(Rotation::Left, amount as u32),
            'R' => ship.change_direction(Rotation::Right, amount as u32),
            'F' => ship.forward(amount as u32),
            _ => unreachable!(),
        }
    }

    println!(
        "{}, {}",
        if ship.x >= 0 {
            format!("East {}", ship.x)
        } else {
            format!("West {}", ship.x.abs())
        },
        if ship.y >= 0 {
            format!("North {}", ship.y)
        } else {
            format!("South {}", ship.y.abs())
        },
    );
    println!("{}", ship.y.abs() + ship.x.abs())
}

pub fn find_waypoint(file: PathBuf) {
    let string: String = read_to_string(file).unwrap();
    let lines: Vec<&str> = string.lines().collect();

    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();
    for line in lines {
        let amount = line[1..].parse::<i32>().unwrap();
        match line.chars().nth(0).unwrap() {
            'N' => {
                waypoint.y += amount;
            }
            'S' => {
                waypoint.y -= amount;
            }
            'E' => {
                waypoint.x += amount;
            }
            'W' => {
                waypoint.x -= amount;
            }
            'L' => waypoint.change_direction(Rotation::Left, amount as u32),
            'R' => waypoint.change_direction(Rotation::Right, amount as u32),
            'F' => ship.to_waypoint(&waypoint, amount as u32),
            _ => unreachable!(),
        }
    }

    println!(
        "{}, {}",
        if ship.x >= 0 {
            format!("East {}", ship.x)
        } else {
            format!("West {}", ship.x.abs())
        },
        if ship.y >= 0 {
            format!("North {}", ship.y)
        } else {
            format!("South {}", ship.y.abs())
        },
    );
    println!("{}", ship.y.abs() + ship.x.abs())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_total() {
        find_manhattan_dist(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day12/test.txt"))
    }

    #[test]
    fn test_waypoint() {
        find_waypoint(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day12/test.txt"))
    }
}
