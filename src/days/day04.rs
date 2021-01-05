struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    exp_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
    valid: bool,
}

pub fn run(part: i32, lines: Vec<String>) {
    let passports = parse_passports(&lines);

    match part {
        1 => part1(&passports),
        2 => part2(&passports),
        _ => {
            part1(&passports);
            part2(&passports);
        }
    }
}

fn part1(passports: &Vec<Passport>) {
    let mut count = 0;
    for p in passports {
        if p.valid {
            count += 1
        };
    }

    println!("{}", count);
}

fn part2(_passports: &Vec<Passport>) {}

fn parse_passports(lines: &Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = vec![];
    let numlines = lines.len();
    let mut i = 0;

    loop {
        if i == numlines {
            break;
        };

        while lines.get(i).unwrap().len() == 0 {
            i += 1;
            if i == numlines {
                break;
            };
        }
        if i == numlines {
            break;
        };

        let mut p = Passport {
            birth_year: None,
            issue_year: None,
            exp_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
            valid: false,
        };

        let mut line = lines.get(i).unwrap();
        while line.len() > 0 {
            let tokens: Vec<&str> = line.split(" ").collect();
            for token in tokens {
                let key_and_value: Vec<&str> = token.split(":").collect();
                let key = key_and_value[0];
                match key {
                    "byr" => {
                        p.birth_year = Some(key_and_value[1].to_string());
                    }
                    "iyr" => {
                        p.issue_year = Some(key_and_value[1].to_string());
                    }
                    "eyr" => {
                        p.exp_year = Some(key_and_value[1].to_string());
                    }
                    "hgt" => {
                        p.height = Some(key_and_value[1].to_string());
                    }
                    "hcl" => {
                        p.hair_color = Some(key_and_value[1].to_string());
                    }
                    "ecl" => {
                        p.eye_color = Some(key_and_value[1].to_string());
                    }
                    "pid" => {
                        p.passport_id = Some(key_and_value[1].to_string());
                    }
                    "cid" => {
                        p.country_id = Some(key_and_value[1].to_string());
                    }
                    _ => {}
                }
            }

            i += 1;
            if i == numlines {
                break;
            };
            line = lines.get(i).unwrap();
        }

        if p.birth_year != None
            && p.exp_year != None
            && p.eye_color != None
            && p.hair_color != None
            && p.height != None
            && p.issue_year != None
            && p.passport_id != None
        {
            p.valid = true
        };
        passports.push(p);
    }

    return passports;
}
