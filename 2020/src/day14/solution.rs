use std::fs::read_to_string;
use std::path::PathBuf;

pub fn sum_memory(file: PathBuf) {
    let string = read_to_string(file).unwrap();
    let lines: Vec<&str> = string.lines().collect();

    let masks: Vec<String> = lines
        .iter()
        .filter(|&x| x.contains("mask"))
        .map(|&x| x.chars().skip(7).collect())
        .collect();
    let mems: Vec<String> = lines
        .iter()
        .filter(|&x| x.contains("mem"))
        .collect::<Vec<&&str>>()
        .iter()
        .map(|&x| {
            //println!("{:?}", x);
            //println!("\n");
            x.to_string()
        })
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_diff() {
        sum_memory(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day14/test.txt"))
    }

    //#[test]
    //fn test_total() {
    //total_diffs(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day10/test.txt"))
    //}
}
