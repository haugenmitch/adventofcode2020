pub fn run(part: i32, lines: Vec<String>) {
    let ids = parse_seats(&lines);

    match part {
        1 => part1(&ids),
        // 2 => part2(&ids),
        _ => {
            part1(&ids);
            // part2(&ids);
        }
    }
}

fn parse_seats(lines: &Vec<String>) -> Vec<i32> {
    let mut ids: Vec<i32> = vec![];
    for line in lines {
        let mut n: i32 = 0;
        for c in line.chars() {
            n = n << 1;
            if c == 'B' || c == 'R' {
                n = n + 1
            };
        }
        ids.push(n);
    }

    ids.sort();
    return ids;
}

fn part1(ids: &Vec<i32>) {
    println!("{}", ids.last().unwrap());
}
