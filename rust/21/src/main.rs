use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::VecDeque;

fn part1(contents: String, start: String) -> String {
    let mut pw: VecDeque<char> = VecDeque::from_iter(start.chars());
    for line in contents.lines() {
        let mut words = line.split_whitespace();

        match words.next().unwrap() {
            "swap" => {
                let (ix1, ix2) = match words.next().unwrap() {
                    "position" => {
                        let ix1: usize = words.next().unwrap().parse().unwrap();
                        words.next();
                        words.next();

                        let ix2: usize = words.next().unwrap().parse().unwrap();
                        (ix1, ix2)
                    },
                    "letter" => {
                        let mut search_char = words.next().unwrap().chars().next().unwrap();
                        let ix1: usize = pw.iter().enumerate().filter_map(|(ix, c)| {
                            if *c == search_char {
                                Some(ix)
                            } else {
                                None
                            }
                        }).next().unwrap();

                        words.next();
                        words.next();

                        search_char = words.next().unwrap().chars().next().unwrap();
                        let ix2: usize = pw.iter().enumerate().filter_map(|(ix, c)| {
                            if *c == search_char {
                                Some(ix)
                            } else {
                                None
                            }
                        }).next().unwrap();
                        (ix1, ix2)
                    },
                    _ => panic!("Unknown operation")
                };
                pw.swap(ix1, ix2);
            },
            "rotate" => {
                match words.next().unwrap() {
                    "based" => {
                        words.next();
                        words.next();
                        words.next();
                        words.next();

                        let search_char = words.next().unwrap().chars().next().unwrap();
                        let mut amt: usize = pw.iter().enumerate().filter_map(|(ix, c)| {
                            if *c == search_char {
                                Some(ix)
                            } else {
                                None
                            }
                        }).next().unwrap() + 1;
                        if amt > 4 {
                            amt += 1;
                        }
                        
                        pw.rotate_right(amt % pw.len());
                    },
                    "left" => {
                        let amt: usize = words.next().unwrap().parse().unwrap();
                        pw.rotate_left(amt % pw.len());
                    },
                    "right" => {
                        let amt: usize = words.next().unwrap().parse().unwrap();
                        pw.rotate_right(amt % pw.len());
                    },
                    _ => panic!("Unknown operation")
                }
            },
            "reverse" => {
                words.next();
                let ix1: usize = words.next().unwrap().parse().unwrap();
                
                words.next();
                let ix2: usize = words.next().unwrap().parse().unwrap();

                let slice: Vec<_> = pw.drain(ix1..=ix2).collect();
                for c in slice.into_iter() {
                    pw.insert(ix1, c);
                }
            },
            "move" => {
                words.next();
                let ix1: usize = words.next().unwrap().parse().unwrap();
                
                words.next();
                words.next();
                let ix2: usize = words.next().unwrap().parse().unwrap();

                let c = pw.remove(ix1).unwrap();
                pw.insert(ix2, c);
            },
            _ => panic!("Unknown operation")
        }
    }
    
    return pw.iter().fold("".to_string(), |mut s, c| {s.push(*c); s});
}

fn part2(contents: String, end: String) -> String {
    let mut pw: VecDeque<char> = VecDeque::from_iter(end.chars());
    for line in contents.lines().rev() {
        let mut words = line.split_whitespace();

        match words.next().unwrap() {
            "swap" => {
                let (ix1, ix2) = match words.next().unwrap() {
                    "position" => {
                        let ix1: usize = words.next().unwrap().parse().unwrap();
                        words.next();
                        words.next();

                        let ix2: usize = words.next().unwrap().parse().unwrap();
                        (ix1, ix2)
                    },
                    "letter" => {
                        let mut search_char = words.next().unwrap().chars().next().unwrap();
                        let ix1: usize = pw.iter().enumerate().filter_map(|(ix, c)| {
                            if *c == search_char {
                                Some(ix)
                            } else {
                                None
                            }
                        }).next().unwrap();

                        words.next();
                        words.next();

                        search_char = words.next().unwrap().chars().next().unwrap();
                        let ix2: usize = pw.iter().enumerate().filter_map(|(ix, c)| {
                            if *c == search_char {
                                Some(ix)
                            } else {
                                None
                            }
                        }).next().unwrap();
                        (ix1, ix2)
                    },
                    _ => panic!("Unknown operation")
                };
                pw.swap(ix1, ix2);
            },
            "rotate" => {
                match words.next().unwrap() {
                    "based" => {
                        words.next();
                        words.next();
                        words.next();
                        words.next();
                        let search_char = words.next().unwrap().chars().next().unwrap();
                        let mut rotated: usize = 0;

                        loop {
                            let mut amt: usize = pw.iter().enumerate().filter_map(|(ix, c)| {
                                if *c == search_char {
                                    Some(ix)
                                } else {
                                    None
                                }
                            }).next().unwrap() + 1;
                            if amt > 4 {
                                amt += 1;
                            }

                            if (amt % pw.len()) == rotated {
                                break;
                            }

                            pw.rotate_left(1);
                            rotated += 1;
                        }
                    },
                    "left" => {
                        let amt: usize = words.next().unwrap().parse().unwrap();
                        pw.rotate_right(amt % pw.len());
                    },
                    "right" => {
                        let amt: usize = words.next().unwrap().parse().unwrap();
                        pw.rotate_left(amt % pw.len());
                    },
                    _ => panic!("Unknown operation")
                }
            },
            "reverse" => {
                words.next();
                let ix1: usize = words.next().unwrap().parse().unwrap();
                
                words.next();
                let ix2: usize = words.next().unwrap().parse().unwrap();

                let slice: Vec<_> = pw.drain(ix1..=ix2).collect();
                for c in slice.into_iter() {
                    pw.insert(ix1, c);
                }
            },
            "move" => {
                words.next();
                let ix1: usize = words.next().unwrap().parse().unwrap();
                
                words.next();
                words.next();
                let ix2: usize = words.next().unwrap().parse().unwrap();

                let c = pw.remove(ix2).unwrap();
                pw.insert(ix1, c);
            },
            _ => panic!("Unknown operation")
        }
    }
    
    return pw.iter().fold("".to_string(), |mut s, c| {s.push(*c); s});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, "abcde".to_string()), "decab".to_string());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "21".to_string();

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
        "\nPart 1:\nScrambled Password: {}\nRan in {:.5?}",
        part1(contents.clone(), "abcdefgh".to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nUnscrambled Password: {}\nRan in {:.5?}",
        part2(contents.clone(), "fbgdceah".to_string()),
        part2_timer.elapsed()
    );
}