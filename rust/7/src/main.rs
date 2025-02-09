use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn part1(contents: String) -> i64 {
    return contents.lines().map(|line| {
        let mut in_brackets = 0;
        let mut past_cs = Vec::new();
        let mut found = false;
        for c in line.chars() {
            match c {
                '[' => {
                    in_brackets += 1;
                    past_cs.clear();
                },
                ']' => {
                    in_brackets -= 1;
                    past_cs.clear();
                },
                _ => {
                    past_cs.push(c);
                    if past_cs.len() == 4 {
                        if past_cs[0] == past_cs[3] && past_cs[1] == past_cs[2] && past_cs[0] != past_cs[1] {
                            if in_brackets > 0 {
                                found = false;
                                break;
                            } else {
                                found = true;
                            }
                        }
                        past_cs.remove(0);
                    }
                },

            }
        }

        found
    }).filter(|&x| x).count() as i64;
}

fn part2(contents: String) -> i64 {
    return contents.lines().map(|line| {
        let mut supernets: HashSet<String> = HashSet::new();
        let mut hypernets: HashSet<String> = HashSet::new();
        let mut in_brackets = 0;
        let mut past_cs = Vec::new();

        for c in line.chars() {
            match c {
                '[' => {
                    in_brackets += 1;
                    past_cs.clear();
                },
                ']' => {
                    in_brackets -= 1;
                    past_cs.clear();
                },
                _ => {
                    past_cs.push(c);
                    if past_cs.len() == 3 {
                        if past_cs[0] == past_cs[2] && past_cs[0] != past_cs[1] {
                            if in_brackets > 0 {
                                hypernets.insert(format!("{}{}{}", past_cs[1], past_cs[0], past_cs[1]));
                            } else {
                                supernets.insert(format!("{}{}{}", past_cs[0], past_cs[1], past_cs[0]));
                            }
                        }
                        past_cs.remove(0);
                    }
                },
            }
        }

        hypernets.intersection(&supernets).count() > 0
    }).filter(|&x| x).count() as i64;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2016".to_string();
    let day = "7".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };


    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nIPs that support TLS: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nIPs that support SSL: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}