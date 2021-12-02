use std::fs::read_to_string;
use std::path::PathBuf;

pub fn break_xmas(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let lines: Vec<i128> = file
        .lines()
        .into_iter()
        .map(|x| x.parse::<i128>().unwrap())
        .collect();

    let mut slice: &[i128];
    let mut index = 0;

    loop {
        slice = &lines[index..=index + 24];

        if !slice
            .iter()
            .any(|&x| slice.contains(&(lines[index + 25] - x)))
        {
            println!("{}", &lines[index + 25]);
            break;
        }
        index += 1;
    }
}

pub fn contiguous_weakness(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let numbers: Vec<i128> = file
        .lines()
        .into_iter()
        .map(|x| x.parse::<i128>().unwrap())
        .collect();

    let target = 1124361034;
    let mut start = 0;
    let mut end = 0;
    let mut sum: i128 = numbers[start..=end].iter().sum();

    loop {
        if sum == target {
            break;
        } else if sum > target {
            sum -= numbers[start];
            start += 1;
        } else {
            end += 1;
            sum += numbers[end];
        }
    }

    println!(
        "{}",
        numbers[start..=end].iter().min().unwrap() + numbers[start..=end].iter().max().unwrap()
    )
}
