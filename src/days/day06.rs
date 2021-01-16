pub fn run(part: i32, lines: Vec<String>) {
    let declarations = parse_declarations(&lines);

    match part {
        1 => part1(&declarations),
        2 => part2(&declarations),
        _ => {
            part1(&declarations);
            part2(&declarations);
        }
    }
}

fn parse_declarations(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut declarations: Vec<Vec<i32>> = vec![];
    let mut group: Vec<i32> = vec![];

    for line in lines {
        if line.is_empty() {
            if !group.is_empty() {
                declarations.push(group);
                group = vec![];
            }
            continue;
        }

        let mut m: i32 = 0;
        for val in line.to_ascii_lowercase().as_bytes().to_vec() {
            m += 1 << (val - 97);
        }
        group.push(m);
    }

    if !group.is_empty() {
        declarations.push(group);
    };

    return declarations;
}

fn count_bits(mut n: i32) -> i32 {
    let mut total: i32 = 0;
    while n > 0 {
        total += n & 1;
        n >>= 1;
    }

    return total;
}

fn part1(declarations: &Vec<Vec<i32>>) {
    let mut total: i32 = 0;
    for dec in declarations {
        let mut group: i32 = 0;
        for n in dec {
            group |= n;
        }
        total += count_bits(group);
    }

    println!("{}", total);
}

fn part2(declarations: &Vec<Vec<i32>>) {
    let mut total: i32 = 0;
    for dec in declarations {
        let mut group: i32 = 0xfffffff;
        for n in dec {
            group &= n;
        }
        total += count_bits(group);
    }

    println!("{}", total);
}
