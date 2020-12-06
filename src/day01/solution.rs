use std::fs;

pub fn sum_to_year() -> u32 {
    // Stupid way to push each line into a vec
    let file = fs::read_to_string("day01/input.txt").unwrap();
    let mut vec: Vec<u32> = Vec::new();
    for fileline in file.lines() {
        vec.push(fileline.parse::<u32>().unwrap());
    }

    for i in &vec {
        if vec.contains(&(2020 - i)) {
            println!("{} {}, {}", i, 2020 - i, (2020 - i) * i);
            return i * (2020 - i);
        }
    }

    0
}

pub fn sum_of_three() -> u32 {
    // Stupid way to push each line into a vec
    let file = fs::read_to_string("day01/input.txt").unwrap();
    let mut vec: Vec<i32> = Vec::new();
    for fileline in file.lines() {
        vec.push(fileline.parse::<i32>().unwrap());
    }

    'outer: for i in &vec {
        for j in &vec {
            let leftover = 2020 - i - j;
            if vec.contains(&leftover) {
                println!("{} {} {}, {}", i, j, leftover, leftover * i * j);
                break 'outer;
            }
        }
    }
    0
}
