use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn traverse(&mut self, x: usize, y: usize) {
        self.x += x;
        self.y += y;
    }
}

pub fn traverse_trees(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let mut trees = 0;

    // Setup a main vector (lines) with subvectors (chars)
    let lines: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

    let mut point = Point { x: 0, y: 0 };
    loop {
        match lines[point.y][point.x] {
            '#' => trees += 1,
            _ => (),
        }

        point.traverse(3, 1);
        if point.x > lines[0].len() - 1 {
            point.x -= lines[0].len();
        }
        if point.y > lines.len() - 1 {
            break;
        }
    }

    println!("{}", trees);
}

pub fn traverse_slopes(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    // Setup a main vector (lines) with subvectors (chars)
    let lines: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut count: u128 = 1;

    for (x, y) in slopes.iter() {
        let mut point = Point { x: 0, y: 0 };
        let mut trees = 0;
        loop {
            match lines[point.y][point.x] {
                '#' => trees += 1,
                _ => (),
            }

            point.traverse(*x, *y);
            if point.x > lines[0].len() - 1 {
                point.x -= lines[0].len();
            }
            if point.y > lines.len() - 1 {
                break;
            }
        }

        count *= trees;
    }
    println!("{}", count);
}
