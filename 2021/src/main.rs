mod day01;

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
    aocday!(day01)
}
