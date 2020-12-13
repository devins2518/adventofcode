use std::path::Path;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("Day 01");
    day01::solution::sum_to_year(path.join("src/day01/input.txt"));
    day01::solution::sum_of_three(path.join("src/day01/input.txt"));
    println!("\nDay 02");
    day02::solution::parse_password(path.join("src/day02/input.txt"));
    day02::solution::parse_password_position(path.join("src/day02/input.txt"));
    println!("\nDay 03");
    day03::solution::traverse_trees(path.join("src/day03/input.txt"));
    day03::solution::traverse_slopes(path.join("src/day03/input.txt"));
    println!("\nDay 04");
    day04::solution::validate_passports(path.join("src/day04/input.txt"));
    day04::solution::valide_passports_constrained(path.join("src/day04/input.txt"));
    println!("\nDay 05");
    day05::solution::find_seat(path.join("src/day05/input.txt"));
    day05::solution::find_my_seat(path.join("src/day05/input.txt"));
    println!("\nDay 06");
    day06::solution::count_groups(path.join("src/day06/input.txt"));
    day06::solution::count_everyone(path.join("src/day06/input.txt"));
}
