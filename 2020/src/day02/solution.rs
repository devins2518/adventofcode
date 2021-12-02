use std::fs::read_to_string;
use std::path::PathBuf;

pub fn parse_password(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let mut valid_pass = 0;

    for line in file.lines() {
        // Parse each line into 0: an array of two values in the range
        //                      1: the char required for the password
        //                      2: the password
        let lex: Vec<&str> = line.split(" ").collect();

        let range: Vec<usize> = lex[0]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let req_char = lex[1].chars().nth(0).unwrap();

        let count = lex[2].matches(req_char).count();

        if range[0] <= count && count <= range[1] {
            valid_pass += 1;
        }
    }

    println!("{}", valid_pass);
}

pub fn parse_password_position(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let mut valid_pass = 0;

    for line in file.lines() {
        // Parse each line into 0: an array of two positions
        //                      1: the char required for the password
        //                      2: the password
        let lex: Vec<&str> = line.split(" ").collect();

        let req_char = lex[1].chars().nth(0).unwrap();

        let positions: Vec<usize> = lex[0]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        // Needs the -1 to use the 1 indexing from the togoggan company
        let first_match = match lex[2].chars().nth(positions[0] - 1) {
            Some(x) => x,
            None => continue,
        };
        let second_match = match lex[2].chars().nth(positions[1] - 1) {
            Some(x) => x,
            None => continue,
        };

        if (first_match == req_char) ^ (second_match == req_char) {
            valid_pass += 1;
        }
    }

    println!("{}", valid_pass);
}
