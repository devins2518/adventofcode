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
