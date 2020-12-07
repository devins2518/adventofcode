use std::fs::read_to_string;

#[derive(Debug)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u16>,
}

impl Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn parse_passport(components: Vec<&str>) -> Self {
        let mut passport = Passport::new();

        for component in components {
            match &component[..=2] {
                "byr" => passport.byr = Some(component[4..].parse::<u16>().unwrap()),
                "iyr" => passport.iyr = Some(component[4..].parse::<u16>().unwrap()),
                "eyr" => passport.eyr = Some(component[4..].parse::<u16>().unwrap()),
                "hgt" => passport.hgt = Some(String::from(&component[4..])),
                "hcl" => passport.hcl = Some(String::from(&component[4..])),
                "ecl" => passport.ecl = Some(String::from(&component[4..])),
                "pid" => passport.pid = Some(String::from(&component[4..])),
                "cid" => passport.cid = Some(component[4..].parse::<u16>().unwrap()),
                _ => panic!(),
            }
        }
        passport
    }

    fn parse_passport_constrained(components: Vec<&str>) -> Self {
        let mut passport = Passport::new();

        for component in components {
            match &component[..=2] {
                "byr" => {
                    let element = component[4..].parse::<u16>().unwrap();
                    passport.byr = match element {
                        1920..=2002 => Some(element),
                        _ => None,
                    }
                }
                "iyr" => {
                    let element = component[4..].parse::<u16>().unwrap();
                    passport.iyr = match element {
                        2010..=2020 => Some(element),
                        _ => None,
                    }
                }
                "eyr" => {
                    let element = component[4..].parse::<u16>().unwrap();
                    passport.eyr = match element {
                        2020..=2030 => Some(element),
                        _ => None,
                    }
                }
                "hgt" => {
                    passport.hgt = {
                        let element = String::from(&component[4..]);
                        let unit = element.split_at(element.len() - 2);
                        match unit.1 {
                            "cm" => {
                                let number = unit.0.parse::<u32>().unwrap();
                                match number {
                                    150..=193 => Some(element),
                                    _ => None,
                                }
                            }
                            "in" => {
                                let number = unit.0.parse::<u32>().unwrap();
                                match number {
                                    59..=76 => Some(element),
                                    _ => None,
                                }
                            }
                            _ => None,
                        }
                    }
                }
                "hcl" => {
                    let element = String::from(&component[4..]);
                    passport.hcl = if element.len() == 7 && element.chars().nth(0).unwrap() == '#' {
                        let hex: String = element
                            .chars()
                            .enumerate()
                            .filter(|i| i.0 != 0)
                            .map(|x| x.1)
                            .collect();
                        if hex.chars().all(|x| match x {
                            'a'..='z' => true,
                            '0'..='9' => true,
                            _ => false,
                        }) {
                            Some(element)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "ecl" => {
                    let element = String::from(&component[4..]);
                    passport.ecl = match element.as_str() {
                        "amb" => Some(element),
                        "blu" => Some(element),
                        "brn" => Some(element),
                        "gry" => Some(element),
                        "grn" => Some(element),
                        "hzl" => Some(element),
                        "oth" => Some(element),
                        _ => None,
                    }
                }
                "pid" => {
                    let element = String::from(&component[4..]);
                    passport.pid = match element.len() {
                        9 => Some(element),
                        _ => None,
                    }
                }
                "cid" => passport.cid = None,
                _ => panic!(),
            }
        }
        passport
    }
}

// Parses the file and returns a vector of Passports to be validated
fn parse_file(constrained: bool) -> Vec<Passport> {
    let file = read_to_string("day04/input.txt").unwrap();
    let groups: Vec<String> = file.split("\n\n").map(|x| x.replace("\n", " ")).collect();

    let mut passports: Vec<Passport> = Vec::new();
    for group in groups {
        let components: Vec<&str> = group.split(" ").filter(|x| x.len() > 0).collect();

        let passport = {
            if constrained {
                Passport::parse_passport_constrained(components)
            } else {
                Passport::parse_passport(components)
            }
        };
        passports.push(passport);
    }

    passports
}

pub fn validate_passports() {
    let passports = parse_file(false);
    let mut count = 0;

    for passport in passports {
        if passport.byr != None
            && passport.iyr != None
            && passport.eyr != None
            && passport.hgt != None
            && passport.hcl != None
            && passport.ecl != None
            && passport.pid != None
        {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn valide_passports_constrained() {
    let passports = parse_file(true);
    let mut count = 0;

    for passport in passports {
        if passport.byr != None
            && passport.iyr != None
            && passport.eyr != None
            && passport.hgt != None
            && passport.hcl != None
            && passport.ecl != None
            && passport.pid != None
        {
            count += 1;
        }
    }
    println!("{}", count);
}
