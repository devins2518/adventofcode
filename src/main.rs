mod day01;
mod day02;

fn main() {
    println!("Day 01");
    day01::solution::sum_to_year();
    day01::solution::sum_of_three();
    println!("\nDay 02");
    day02::solution::parse_password();
    day02::solution::parse_password_position();
}
