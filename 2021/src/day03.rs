pub fn part1(s: &str) -> usize {
    let split = s.split_terminator('\n');
    let gamma_rate: String = split
        .fold(Vec::new(), |mut acc: Vec<isize>, s| {
            for (i, x) in s.chars().enumerate() {
                match (x, acc.get_mut(i)) {
                    (_, None) => acc.push(0),
                    ('1', Some(x)) => *x += 1,
                    ('0', Some(x)) => *x -= 1,
                    _ => unreachable!(),
                }
            }
            acc
        })
        .iter()
        .map(|x| if *x <= 0 { '0' } else { '1' })
        .collect();
    let epsilon_rate: String = gamma_rate
        .chars()
        .map(|x| if x == '0' { '1' } else { '0' })
        .collect();

    usize::from_str_radix(&gamma_rate, 2).unwrap()
        * usize::from_str_radix(&epsilon_rate, 2).unwrap()
}

pub fn part2(s: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010\
        ";
        assert_eq!(part1(s), 198);
        assert_eq!(part2(s), 230);
    }
}
