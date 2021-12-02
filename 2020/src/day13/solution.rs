use std::fs::read_to_string;
use std::path::PathBuf;

pub fn find_wait(file: PathBuf) {
    let string = read_to_string(file).unwrap();
    let lines: Vec<&str> = string.lines().collect();

    let time = lines[0].parse::<u32>().unwrap();
    let bus_times: Vec<u32> = lines[1]
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut selected_time: u32 = time;
    let id: u32;

    'outer: loop {
        for i in &bus_times {
            if selected_time % i == 0 {
                id = *i;
                break 'outer;
            }
        }
        selected_time += 1;
    }

    println!("{}", ((selected_time - time) * id));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_wait() {
        find_wait(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day13/test.txt"))
    }

    //#[test]
    //fn test_total() {
    //total_diffs(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/day10/test.txt"))
    //}
}
