use std::fs;
use std::path::PathBuf;

fn parse_input(file: PathBuf) -> Vec<(String, String)> {
    let input = fs::read_to_string(file).unwrap();
    // Filter out empty string and empty bags
    let input = input
        .split("\n")
        .filter(|x| x != &"" && !x.contains("no other bags"));

    let bags: Vec<(String, String)> = input
        .map(|x| {
            let split: Vec<&str> = x.split(" contain ").collect();
            (split[0].to_string(), split[1].to_string())
        })
        .collect();

    bags
}

pub fn contains_single_shiny(file: PathBuf) {
    let bags = parse_input(file);
    println!("{:#?}", bags);
}
