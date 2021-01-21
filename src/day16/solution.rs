use std::fs;
use std::path::PathBuf;

pub fn find_error_rate(file: PathBuf) {
    let string = fs::read_to_string(file).unwrap();
    let lines: Vec<&str> = string.split("\n\n").collect();

    let rules: Vec<&str> = lines[0].split("\n").collect();
    let nearby: Vec<Vec<u32>> = lines[2]
        .split("\n")
        .skip(1)
        .filter(|&x| x != "")
        .map(|x| {
            let split: Vec<&str> = x.split(",").filter(|&x| x != "").collect();
            split.iter().map(|x| x.parse::<u32>().unwrap()).collect()
        })
        .collect();

    println!("{:?}", nearby);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_error_rate() {
        find_error_rate(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day16/test.txt"))
    }

    //#[test]
    //fn test_total() {
    //total_diffs(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day10/test.txt"))
    //}
}
