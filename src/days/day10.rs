pub fn run(part: i32, lines: Vec<String>) {
    let mut vec: Vec<i64> = vec![];
    for line in lines {
        match line.trim().parse::<i64>() {
            Ok(n) => vec.push(n),
            Err(_) => println!("\"{}\" could not be parsed as an integer", line),
        }
    }

    vec.push(0);
    vec.sort();
    vec.push(vec.iter().max().unwrap() + 3);

    match part {
        1 => part1(&vec),
        2 => part2(&vec),
        _ => {
            part1(&vec);
            part2(&vec);
        }
    }
}

fn part1(vec: &Vec<i64>) {
    let mut diff1 = 0;
    let mut diff3 = 0;

    for i in 1..vec.len() {
        let diff = vec[i] - vec[i - 1];
        if diff == 1 {
            diff1 += 1
        } else if diff == 3 {
            diff3 += 1
        };
    }

    println!("{}", diff1 * diff3);
}

fn part2(vec: &Vec<i64>) {
    let mut options: Vec<i64> = vec![0; vec.len()];

    options[0] = 1;

    for i in 1..vec.len() {
        let mut total = 0;
        total += options[i - 1];
        if ((i as i64) - 2) >= 0 && (vec[i - 2] + 3) >= vec[i] {
            total += options[i - 2];
        }
        if ((i as i64) - 3) >= 0 && (vec[i - 3] + 3) >= vec[i] {
            total += options[i - 3];
        }

        options[i] = total;
    }

    println!("{}", options[vec.len() - 1]);
}
