use std::fs::read_to_string;
use std::path::PathBuf;

struct SeatLayout {
    layout: Vec<Vec<Option<bool>>>,
}

impl std::fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::with_capacity(self.layout.len() + self.layout[0].len());
        for i in &self.layout {
            for j in i {
                string.push(match j {
                    Some(true) => 'L',
                    Some(false) => '#',
                    None => '.',
                })
            }
            string.push('\n');
        }
        string.push('\n');
        write!(f, "{}", string)
    }
}

impl SeatLayout {
    fn new(lines: Vec<&str>) -> Self {
        let layout: Vec<Vec<Option<bool>>> = lines
            .iter()
            // Build outside vector of Vec<Options>
            .map(|&x| {
                // Build inside vector of Options
                x.chars()
                    .map(|c| match c {
                        // True if empty
                        'L' => Some(true),
                        '#' => Some(false),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        SeatLayout { layout }
    }

    fn find_adjacent(&self) -> u8 {
        unimplemented!()
    }

    fn apply(&mut self, grid: Vec<Vec<Option<bool>>>) {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                self.layout[i][j] = grid[i][j];
            }
        }
    }

    fn clock(&mut self) {}
}

pub fn seat_occupied(file: PathBuf) {
    let string = read_to_string(file).unwrap();
    let lines: Vec<&str> = string.lines().collect();

    let layout = SeatLayout::new(lines);
    println!("{}", layout);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_seats() {
        seat_occupied(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day11/test.txt"))
    }
}
