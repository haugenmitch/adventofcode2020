struct Slope {
    right: usize,
    down: usize,
}

pub fn run(part: i32, lines: Vec<String>) {
    match part {
        1 => part1(&lines),
        2 => part2(&lines),
        _ => {
            part1(&lines);
            part2(&lines);
        }
    }
}

fn part1(lines: &Vec<String>) {
    println!("{}", count_tree_hits(Slope { right: 3, down: 1 }, lines));
}

fn part2(lines: &Vec<String>) {
    let n1: i64 = count_tree_hits(Slope { right: 1, down: 1 }, lines) as i64;
    let n2: i64 = count_tree_hits(Slope { right: 3, down: 1 }, lines) as i64;
    let n3: i64 = count_tree_hits(Slope { right: 5, down: 1 }, lines) as i64;
    let n4: i64 = count_tree_hits(Slope { right: 7, down: 1 }, lines) as i64;
    let n5: i64 = count_tree_hits(Slope { right: 1, down: 2 }, lines) as i64;

    println!("{}", n1 * n2 * n3 * n4 * n5);
}

fn count_tree_hits(slope: Slope, lines: &Vec<String>) -> i32 {
    let line_length = lines.first().unwrap().len();
    let mut count = 0;
    let mut x = 0;
    for i in 0..lines.len() {
        if i % slope.down == 0 {
            if lines.get(i).unwrap().chars().nth(x).unwrap() == '#' {
                count += 1;
            }
            x = (x + slope.right) % line_length;
        }
    }

    return count;
}
