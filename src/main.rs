mod day01;
mod day02;
mod day03;

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
}
