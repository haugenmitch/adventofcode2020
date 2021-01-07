struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
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

fn part2(passports: &Vec<Passport>) {
    let mut count = 0;
    for p in passports {
        if check_byr(&p.byr)
            && check_iyr(&p.iyr)
            && check_eyr(&p.eyr)
            && check_hgt(&p.hgt)
            && check_hcl(&p.hcl)
            && check_ecl(&p.ecl)
            && check_pid(&p.pid)
        {
            count += 1
        };
    }

    println!("{}", count);
}

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
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
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
                        p.byr = Some(key_and_value[1].to_string());
                    }
                    "iyr" => {
                        p.iyr = Some(key_and_value[1].to_string());
                    }
                    "eyr" => {
                        p.eyr = Some(key_and_value[1].to_string());
                    }
                    "hgt" => {
                        p.hgt = Some(key_and_value[1].to_string());
                    }
                    "hcl" => {
                        p.hcl = Some(key_and_value[1].to_string());
                    }
                    "ecl" => {
                        p.ecl = Some(key_and_value[1].to_string());
                    }
                    "pid" => {
                        p.pid = Some(key_and_value[1].to_string());
                    }
                    "cid" => {
                        p.cid = Some(key_and_value[1].to_string());
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

        if p.byr != None
            && p.eyr != None
            && p.ecl != None
            && p.hcl != None
            && p.hgt != None
            && p.iyr != None
            && p.pid != None
        {
            p.valid = true
        };
        passports.push(p);
    }

    return passports;
}

fn check_byr(s: &Option<String>) -> bool {
    match s {
        None => return false,
        Some(val) => match val.parse::<i32>() {
            Ok(n) => {
                if 1920 <= n && n <= 2002 {
                    return true;
                };
            }
            Err(_) => return false,
        },
    }
    return false;
}

fn check_iyr(s: &Option<String>) -> bool {
    match s {
        None => return false,
        Some(val) => match val.parse::<i32>() {
            Ok(n) => {
                if 2010 <= n && n <= 2020 {
                    return true;
                };
            }
            Err(_) => return false,
        },
    }
    return false;
}

fn check_eyr(s: &Option<String>) -> bool {
    match s {
        None => return false,
        Some(val) => match val.parse::<i32>() {
            Ok(n) => {
                if 2020 <= n && n <= 2030 {
                    return true;
                };
            }
            Err(_) => return false,
        },
    }
    return false;
}

fn check_hgt(s: &Option<String>) -> bool {
    match s {
        None => return false,
        Some(val) => {
            if val.ends_with("cm") {
                match val[..(val.len() - 2)].parse::<i32>() {
                    Ok(n) => {
                        if 150 <= n && n <= 193 {
                            return true;
                        }
                    }
                    Err(_) => return false,
                }
            } else if val.ends_with("in") {
                match val[..(val.len() - 2)].parse::<i32>() {
                    Ok(n) => {
                        if 59 <= n && n <= 76 {
                            return true;
                        }
                    }
                    Err(_) => return false,
                }
                return false;
            }
        }
    }
    return false;
}

fn check_hcl(s: &Option<String>) -> bool {
    match s {
        None => return false,
        Some(val) => {
            if val.len() == 7
                && val.starts_with("#")
                && val[1..7].chars().all(|x| x.is_ascii_hexdigit())
            {
                return true;
            }
        }
    }
    return false;
}

fn check_ecl(s: &Option<String>) -> bool {
    match s {
        None => return false,
        Some(val) => {
            if val == "amb"
                || val == "blu"
                || val == "brn"
                || val == "gry"
                || val == "grn"
                || val == "hzl"
                || val == "oth"
            {
                return true;
            }
        }
    }
    return false;
}

fn check_pid(s: &Option<String>) -> bool {
    match s {
        None => return false,
        Some(val) => {
            if val.len() == 9 {
                match val.parse::<i32>() {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
        }
    }
    return false;
}
