pub fn part1(s: &str) -> usize {
    s.split('\n')
        .map(|x| x.parse().unwrap_or(0))
        .collect::<Vec<usize>>()
        .windows(2)
        .fold(0, |acc, x| if x[0] < x[1] { acc + 1 } else { acc })
}

pub fn part2(s: &str) -> usize {
    s.split('\n')
        .map(|x| x.parse().unwrap_or(0))
        .collect::<Vec<usize>>()
        .windows(4)
        .fold(0, |acc, x| {
            let mut w = x.windows(3);
            let a: usize = w.next().unwrap().iter().sum();
            let b: usize = w.next().unwrap().iter().sum();
            if a < b {
                acc + 1
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(part1(s), 7);
        assert_eq!(part2(s), 5);
    }
}
