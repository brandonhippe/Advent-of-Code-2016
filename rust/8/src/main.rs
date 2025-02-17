use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn get_display(contents: String) -> HashSet<(i64, i64)> {
    let mut screen: HashSet<(i64, i64)> = HashSet::new();
    let width = 50;
    let height = 6;
    
    for line in contents.lines() {
        let command = line.split_whitespace().collect::<Vec<&str>>();
        match command[0] {
            "rect" => {
                let dims = command[1]
                    .split('x')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|&s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
    
                for x in 0..dims[0] {
                    for y in 0..dims[1] {
                        screen.insert((x, y));
                    }
                }
            },
            "rotate" => {
                let index: i64 = command[2].split('=').collect::<Vec<&str>>()[1]
                    .parse::<i64>()
                    .unwrap();
                let shift: i64 = command[4].parse::<i64>().unwrap();
                let mut removing: HashSet<(i64, i64)> = HashSet::new();
                let mut adding: HashSet<(i64, i64)> = HashSet::new();
    
                match command[1] {
                    "row" => {
                        for x in 0..width {
                            if screen.contains(&(x, index)) {
                                removing.insert((x, index));
                                let new_x = (x + shift) % width;
                                adding.insert((new_x, index));
                            }
                        }
                    },
                    "column" => {
                        for y in 0..height {
                            if screen.contains(&(index, y)) {
                                removing.insert((index, y));
                                let new_y = (y + shift) % height;
                                adding.insert((index, new_y));
                            }
                        }
                    },
                    _ => eprintln!("Unknown command: {}", command[1]),
                }
    
                screen = screen
                    .difference(&removing)
                    .cloned()
                    .collect::<HashSet<(i64, i64)>>();
                screen = screen
                    .union(&adding)
                    .cloned()
                    .collect::<HashSet<(i64, i64)>>();
                
            },
            _ => eprintln!("Unknown command: {}", command[0]),
        }
    }

    return screen;
}

fn part1(contents: String) -> i64 {
    return get_display(contents).len() as i64;
}

fn part2(contents: String) -> String {
    let screen = get_display(contents);
    let mut output = String::new();

    for y in 0..6 {
        for x in 0..50 {
            if screen.contains(&(x, y)) {
                output.push('█');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 0);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "8".to_string();

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
        "\nPart 1:\nOn pixels: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMessage:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}