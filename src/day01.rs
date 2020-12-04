use std::fs;

pub fn sum_to_year() -> u32 {
    let file = fs::read_to_string("input_day01.txt").unwrap();
    let mut vec: Vec<u32> = Vec::new();
    for fileline in file.lines() {
        vec.push(fileline.parse::<u32>().unwrap());
    }

    for (idx, i) in vec.iter().enumerate() {
        for j in vec[idx + 1..].iter() {
            if i + j == 2020 {
                println!("{} {}, {}", i, j, i * j);
                return i * j;
            }
        }
    }
    0
}

pub fn sum_of_three() -> u32 {
    let file = fs::read_to_string("input_day01.txt").unwrap();
    let mut vec: Vec<u32> = Vec::new();
    for fileline in file.lines() {
        vec.push(fileline.parse::<u32>().unwrap());
    }

    for (idx, i) in vec.iter().enumerate() {
        for j in vec[idx + 1..].iter() {
            for k in vec[idx + 2..].iter() {
                if i + j + k == 2020 {
                    println!("{} {} {}, {}", i, j, k, i * k * j);
                    return i * k * j;
                }
            }
        }
    }
    0
}
