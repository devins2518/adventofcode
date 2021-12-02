use std::fs::read_to_string;
use std::path::PathBuf;

struct Range {
    max: u8,
    min: u8,
    result: u8,
}

impl Range {
    fn new_row() -> Self {
        Range {
            max: 127,
            min: 0,
            result: 0,
        }
    }
    fn new_column() -> Self {
        Range {
            max: 7,
            min: 0,
            result: 0,
        }
    }

    fn upper_half(&mut self) {
        self.min = (self.min + self.max) / 2 + 1;
        if self.max - self.min == 1 {
            self.result = self.max;
        }
    }
    fn lower_half(&mut self) {
        self.max = (self.min + self.max) / 2;
        if self.max - self.min == 1 {
            self.result = self.min;
        }
    }
}

fn parse_chars(chars: Vec<char>, row: &mut Range, column: &mut Range) {
    for character in chars {
        match character {
            'F' => row.lower_half(),
            'B' => row.upper_half(),
            'L' => column.lower_half(),
            'R' => column.upper_half(),
            _ => panic!(),
        }
    }
}

pub fn find_seat(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let seats: Vec<&str> = file.split("\n").filter(|x| x.len() > 0).collect();

    let mut high = 0u64;

    for seat in seats {
        let mut row = Range::new_row();
        let mut column = Range::new_column();
        parse_chars(seat.chars().collect(), &mut row, &mut column);

        let seat_id: u64 = (row.result as u64 * 8) + column.result as u64;
        if seat_id > high {
            high = seat_id;
        }
    }

    println!("{}", high);
}

pub fn find_my_seat(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let mut boarding_passes: Vec<_> = file
        .lines()
        .map(|l| {
            l.chars().fold(0, |acc, half| {
                (acc << 1) | if half == 'B' || half == 'R' { 1 } else { 0 }
            })
        })
        .collect();
    boarding_passes.sort();

    boarding_passes.sort();

    let result = (boarding_passes[0]..=boarding_passes[boarding_passes.len() - 1])
        .zip(boarding_passes.iter())
        .find(|(expected, sead_id)| expected != *sead_id)
        .expect("input is not empty");

    println!("{}", result.0);
}
