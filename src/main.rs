mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    println!("Day 01");
    day01::solution::sum_to_year();
    day01::solution::sum_of_three();
    println!("\nDay 02");
    day02::solution::parse_password();
    day02::solution::parse_password_position();
    println!("\nDay 03");
    day03::solution::traverse_trees();
    day03::solution::traverse_slopes();
    println!("\nDay 04");
    day04::solution::validate_passports();
    day04::solution::valide_passports_constrained();
    println!("\nDay 05");
    day05::solution::find_seat();
    day05::solution::find_my_seat();
}
