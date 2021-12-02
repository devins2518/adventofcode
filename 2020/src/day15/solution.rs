use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn parse_input(file: PathBuf) -> Vec<u32> {
    let file = fs::read_to_string(file).unwrap();
    let mut numbers: Vec<u32> = file.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    numbers
}

pub fn number_spoken(file: PathBuf) {
    let numbers = parse_input(file);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_diff() {
        number_spoken(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day10/test.txt"))
    }
}
