use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let pos_mapping: HashMap<(i64, i64), i64> = HashMap::from_iter(vec![
        ((0, 0), 1),
        ((1, 0), 2),
        ((2, 0), 3),
        ((0, 1), 4),
        ((1, 1), 5),
        ((2, 1), 6),
        ((0, 2), 7),
        ((1, 2), 8),
        ((2, 2), 9),
    ]);

    return contents
        .lines()
        .fold(((1, 1), "".to_string()), |acc, line| {
            let new_pos = line.chars().fold(acc.0, |pos, c| {
                let new_pos = match c {
                    'U' => (pos.0, pos.1 - 1),
                    'D' => (pos.0, pos.1 + 1),
                    'L' => (pos.0 - 1, pos.1),
                    'R' => (pos.0 + 1, pos.1),
                    _ => panic!("Invalid direction"),
                };

                if pos_mapping.contains_key(&new_pos) {
                    return new_pos;
                } else {
                    return pos;

                }
            });

            return (
                new_pos,
                acc.1 + pos_mapping.get(&new_pos).unwrap().to_string().as_str(),
            );
        })
        .1
        .parse::<i64>()
        .unwrap();
}

fn part2(contents: String) -> String {
    let pos_mapping: HashMap<(i64, i64), char> = HashMap::from_iter(vec![
        ((1, -1), '1'),
        ((0, 0), '2'),
        ((1, 0), '3'),
        ((2, 0), '4'),
        ((-1, 1), '5'),
        ((0, 1), '6'),
        ((1, 1), '7'),
        ((2, 1), '8'),
        ((3, 1), '9'),
        ((0, 2), 'A'),
        ((1, 2), 'B'),
        ((2, 2), 'C'),
        ((1, 3), 'D'),
    ]);

    return contents
        .lines()
        .fold(((-1, 1), "".to_string()), |acc, line| {
            let new_pos = line.chars().fold(acc.0, |pos, c| {
                let new_pos = match c {
                    'U' => (pos.0, pos.1 - 1),
                    'D' => (pos.0, pos.1 + 1),
                    'L' => (pos.0 - 1, pos.1),
                    'R' => (pos.0 + 1, pos.1),
                    _ => panic!("Invalid direction"),
                };

                if pos_mapping.contains_key(&new_pos) {
                    return new_pos;
                } else {
                    return pos;
                }
            });

            return (
                new_pos,
                acc.1 + pos_mapping.get(&new_pos).unwrap().to_string().as_str(),
            );
        })
        .1
        .to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents = "ULL\nRRDDD\nLURDL\nUUUUD\n".to_string();
        assert_eq!(part1(contents), 1985);
    }

    #[test]
    fn p2_test() {
        let contents = "ULL\nRRDDD\nLURDL\nUUUUD\n".to_string();
        assert_eq!(part2(contents), "5DB3".to_string());
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2016".to_string();
    let day = "2".to_string();

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
        "\nPart 1:\nCode: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCode: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}