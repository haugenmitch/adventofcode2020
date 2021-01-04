pub fn run(part: i32, lines: Vec<String>) {
    match part {
        1 => part1(&lines),
        // 2 => part2(&lines),
        _ => {
            part1(&lines);
            // part2(&lines);
        }
    }
}

fn part1(lines: &Vec<String>) {
    let l = lines.first().unwrap().len();
    let mut i: usize = 0;
    let mut count = 0;

    for line in lines {
        if line.chars().nth(i).unwrap() == '#' {
            count += 1;
        }
        i = (i + 3) % l;
    }

    println!("{}", count);
}
