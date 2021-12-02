use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn parse_input(file: PathBuf) -> Vec<u32> {
    let file = fs::read_to_string(file).unwrap();
    let mut numbers: Vec<u32> = file.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);
    numbers
}

pub fn joltage_diff(file: PathBuf) {
    let numbers = parse_input(file);

    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 0;

    let mut input_joltage = 0;

    for i in numbers {
        let output_joltage = i;

        match output_joltage - input_joltage {
            1 => one_jolt_diff += 1,
            3 => three_jolt_diff += 1,
            _ => unreachable!(),
        }

        input_joltage = i;
    }

    println!("{}", one_jolt_diff * three_jolt_diff);
}

pub fn total_diffs(file: PathBuf) {
    let numbers = parse_input(file);
    let mut arranges: HashMap<u32, u128> = HashMap::new();
    arranges.insert(0, 1);

    for i in &numbers {
        arranges.insert(
            *i,
            arranges.get(&(i.wrapping_sub(3))).unwrap_or(&0)
                + arranges.get(&(i.wrapping_sub(2))).unwrap_or(&0)
                + arranges.get(&(i.wrapping_sub(1))).unwrap_or(&0),
        );
    }

    println!("{:?}", arranges.get(&numbers.last().unwrap()).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_diff() {
        joltage_diff(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day10/test.txt"))
    }

    #[test]
    fn test_total() {
        total_diffs(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day10/test.txt"))
    }
}
