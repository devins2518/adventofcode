pub fn part1(s: &str) -> usize {
    let (h, d) = s
        .split_terminator('\n')
        .map(|x| {
            let mut d = x.split_whitespace();
            (
                d.next().unwrap(),
                d.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold((0, 0), |mut acc, x| {
            match x.0 {
                "forward" => acc.0 += x.1,
                "up" => acc.1 -= x.1,
                "down" => acc.1 += x.1,
                _ => unreachable!(),
            };
            acc
        });

    h * d
}

pub fn part2(s: &str) -> usize {
    let (h, d, _) = s
        .split_terminator('\n')
        .map(|x| {
            let mut d = x.split_whitespace();
            (
                d.next().unwrap(),
                d.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold((0, 0, 0), |mut acc, x| {
            match x.0 {
                "forward" => {
                    acc.0 += x.1;
                    acc.1 += acc.2 * x.1;
                }
                "up" => acc.2 -= x.1,
                "down" => acc.2 += x.1,
                _ => unreachable!(),
            };
            acc
        });

    h * d
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2\
        ";
        assert_eq!(part1(s), 150);
        assert_eq!(part2(s), 900);
    }
}
