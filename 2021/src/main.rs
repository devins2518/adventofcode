mod day01;
mod day02;
mod day03;

macro_rules! aocday {
    ($day: ident) => {{
        use std::fs::read_to_string;
        println!("{}:", stringify!($day));
        let s = read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/",
            stringify!($day),
            ".txt"
        ))
        .unwrap();
        println!("{}", $day::part1(&s));
        println!("{}", $day::part2(&s));
    }};
}

fn main() {
    aocday!(day01);
    aocday!(day02);
    aocday!(day03);
}
