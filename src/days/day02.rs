struct Record {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

pub fn run(part: i32, lines: Vec<String>) {
    let records = parse_password_info(lines);

    match part {
        1 => part1(&records),
        2 => part2(&records),
        _ => {
            part1(&records);
            part2(&records);
        }
    }
}

fn parse_password_info(lines: Vec<String>) -> Vec<Record> {
    let mut vec: Vec<Record> = vec![];
    for line in lines {
        let tokens: Vec<&str> = line.split(" ").collect();
        let minmax: Vec<&str> = tokens[0].split("-").collect();
        let min: usize = minmax[0].to_string().parse().unwrap();
        let max: usize = minmax[1].to_string().parse().unwrap();
        let letter: char = tokens[1].chars().next().unwrap();
        let password: String = tokens[2].to_string();
        vec.push(Record {
            min,
            max,
            letter,
            password,
        })
    }
    return vec;
}

fn part1(records: &Vec<Record>) {
    let mut success = 0;
    for rec in records {
        let count = rec.password.matches(rec.letter).count();
        if rec.min <= count && count <= rec.max {
            success += 1;
        }
    }
    println!("{}", success);
}

fn part2(records: &Vec<Record>) {
    let mut count = 0;
    for rec in records {
        count += if check2(rec) { 1 } else { 0 };
    }
    println!("{}", count);
}

fn check2(record: &Record) -> bool {
    let c1 = record.password.chars().nth(record.min - 1).unwrap();
    let c2 = record.password.chars().nth(record.max - 1).unwrap();

    return (c1 == record.letter) ^ (c2 == record.letter);
}
