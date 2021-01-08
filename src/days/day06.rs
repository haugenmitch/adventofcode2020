pub fn run(part: i32, lines: Vec<String>) {
    let responses = parse_customs(&lines);

    match part {
        1 => part1(&responses),
        2 => part2(&responses),
        _ => {
            part1(&responses);
            part2(&responses);
        }
    }
}

fn parse_customs(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut responses: Vec<Vec<char>> = vec![];
    let mut group: Vec<char> = vec![];

    for line in lines {
        if line.is_empty() {
            if group.len() > 0 {
                group.sort();
                group.dedup();
                responses.push(group);
            }
            group = vec![];
            continue;
        };

        group.append(&mut line.chars().collect::<Vec<_>>());
    }
    if group.len() > 0 {
        group.sort();
        group.dedup();
        responses.push(group);
    }

    return responses;
}

fn part1(responses: &Vec<Vec<char>>) {
    let mut total = 0;
    for response in responses {
        total += response.len();
    }

    println!("{}", total);
}

fn part2(_responses: &Vec<Vec<char>>) {}
