pub fn run(part: i32, lines: Vec<String>) {
    let mut vec: Vec<i64> = vec![];
    for line in lines {
        match line.trim().parse::<i64>() {
            Ok(n) => vec.push(n),
            Err(_) => println!("\"{}\" could not be parsed as an integer", line),
        }
    }

    if vec.len() < 25 {
        return;
    };

    match part {
        1 => part1(&vec),
        2 => part2(&vec),
        _ => {
            part1(&vec);
            part2(&vec);
        }
    }
}

fn compose(n: i64, vec: &Vec<i64>) -> bool {
    for i in 0..vec.len() {
        if vec[i] > n {
            return false;
        };

        for j in i + 1..vec.len() {
            if vec[i] + vec[j] == n {
                return true;
            } else if vec[i] + vec[j] > n {
                break;
            }
        }
    }
    return false;
}

fn part1(vec: &Vec<i64>) {
    let mut buffer: Vec<i64> = vec[..25].to_vec();
    let mut buffer_sorted: Vec<i64> = vec[..25].to_vec();
    buffer_sorted.sort();

    for i in 25..vec.len() {
        let n = vec[i];

        if !compose(n, &buffer_sorted) {
            println!("{}", n);
            return;
        }

        buffer_sorted.remove(buffer_sorted.binary_search(&buffer[0]).unwrap());
        buffer.remove(0);

        buffer.push(n);
        buffer_sorted.insert(buffer_sorted.binary_search(&n).unwrap_or_else(|e| e), n);
    }
}

fn part2(vec: &Vec<i64>) {
    let target = 731031916;
    let mut running_total: i64 = 0;
    let mut buffer: Vec<i64> = vec![];

    for i in vec {
        buffer.push(*i);
        running_total += i;

        if running_total < target {
            continue;
        }

        while running_total > target {
            running_total -= buffer[0];
            buffer.remove(0);
        }

        if running_total == target {
            println!(
                "{}",
                buffer.iter().min().unwrap() + buffer.iter().max().unwrap()
            );
            break;
        }
    }
}
