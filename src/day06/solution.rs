use std::path::PathBuf;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn count_questions(string: &str) -> HashSet<char> {
    let mut hash = HashSet::new();

    for character in string.chars() {
        if character != '\n' {
            hash.insert(character);
        }
    }
    hash
}

pub fn count_groups(input: PathBuf) {
    let file = read_to_string(input).unwrap();
    let lines: Vec<&str> = file.split("\n\n").collect();

    let mut group_count = 0;

    for line in lines {
        group_count += count_questions(line).len();
    }

    println!("{:?}", group_count);
}

fn count_questions_collection(string: &str) -> u32 {
    let groups = string.replace("\n", " ");
    let people: Vec<&str> = groups.split(" ").filter(|&x| x != "").collect();

    let mut hash: HashMap<char, usize> = HashMap::new();

    for person in &people {
        for character in person.chars() {
            if hash.contains_key(&character) {
                *hash.get_mut(&character).unwrap() += 1;
            } else {
                hash.insert(character, 1);
            }
        }
    }

    let mut length = 0;

    println!("{:?}", people);
    for element in hash {
        println!("{:?}", element);
        if element.1 == people.len() {
            length += 1;
        }
    }

    length
}

pub fn count_everyone(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let lines: Vec<&str> = file.split("\n\n").collect();

    let mut collective = 0;
    for line in lines {
        collective += count_questions_collection(line);
    }

    println!("{}", collective);
}
