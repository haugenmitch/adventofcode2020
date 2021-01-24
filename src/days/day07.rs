struct Bag {
    color: String,
    inner_bags: Vec<(String, i32)>,
}

pub fn run(part: i32, lines: Vec<String>) {
    let bags = parse_bags(&lines);

    match part {
        1 => part1(&bags),
        2 => part2(&bags),
        _ => {
            part1(&bags);
            part2(&bags);
        }
    }
}

fn parse_bags(lines: &Vec<String>) -> Vec<Bag> {
    let mut bags: Vec<Bag> = vec![];

    for line in lines {
        let mut tokens: Vec<&str> = line.split("bags contain").collect();
        let color: String = tokens[0].trim().to_string();
        if tokens[1] == " no other bags.".to_string() {
            bags.push(Bag {
                color: color,
                inner_bags: vec![],
            });
            continue;
        }

        let sub_bags_string = tokens[1]
            .replace("bags", "")
            .replace("bag", "")
            .replace(".", "")
            .trim()
            .to_string();
        let sub_bag_list: Vec<&str> = sub_bags_string.split(" , ").collect();

        let mut sub_bags: Vec<(String, i32)> = vec![];
        for sub_bag in sub_bag_list {
            tokens = sub_bag.splitn(2, " ").collect();
            sub_bags.push((
                tokens[1].trim().to_string(),
                tokens[0].parse::<i32>().unwrap(),
            ));
        }

        bags.push(Bag {
            color: color.clone(),
            inner_bags: sub_bags,
        });
    }

    return bags;
}

fn part1(bags: &Vec<Bag>) {
    let mut good: Vec<String> = ["shiny gold".to_string()].to_vec();
    let mut bad: Vec<String> = vec![];

    for bag in bags {
        let (_success, a, b) = recursive_search(bags, &bag.color, good.clone(), bad.clone());
        good = a;
        bad = b;
    }

    println!("{}", good.len() - 1); // don't count the shiny gold bag
}

fn recursive_search(
    bags: &Vec<Bag>,
    color: &String,
    mut good: Vec<String>,
    mut bad: Vec<String>,
) -> (bool, Vec<String>, Vec<String>) {
    if good.contains(color) {
        return (true, good.clone(), bad.clone());
    } else if bad.contains(color) {
        return (false, good.clone(), bad.clone());
    }

    for bag in bags {
        if color == &bag.color {
            for inner_bag in &bag.inner_bags {
                let (a, b, c) =
                    recursive_search(bags, &inner_bag.0.to_string(), good.clone(), bad.clone());
                good = b;
                bad = c;
                if a {
                    good.push(color.to_string());
                    return (true, good.clone(), bad.clone());
                }
            }
        }
    }

    bad.push(color.to_string());
    return (false, good.clone(), bad.clone());
}

fn part2(bags: &Vec<Bag>) {
    println!("{}", recursive_count(bags, "shiny gold".to_string()));
}

fn recursive_count(bags: &Vec<Bag>, color: String) -> i64 {
    let mut count: i64 = 0;

    for bag in bags {
        if bag.color == color {
            for sub_bag in &bag.inner_bags {
                count += sub_bag.1 as i64
                    + (sub_bag.1 as i64) * recursive_count(bags, sub_bag.0.clone());
            }
        }
    }

    return count;
}
