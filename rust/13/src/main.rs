use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, VecDeque};

fn space(pos: (i64, i64), num: i64) -> bool {
    let x = pos.0;
    let y = pos.1;

    let mut test_val = x*x + 3*x + 2*x*y + y + y*y + num;
    let mut out: bool = true;
    while test_val > 0 {
        out ^= (test_val & 1) != 0;
        test_val >>= 1;
    }

    out
}

fn part1(contents: String, goal_pos: (i64, i64)) -> i64 {
    let mut open_spaces: HashSet<(i64, i64)> = HashSet::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let num: i64 = contents.trim().parse().unwrap();

    let mut to_check: VecDeque<(i64, (i64, i64))> = VecDeque::from(vec![(0, (1, 1))]);
    while let Some((dist, pos)) = to_check.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        if pos == goal_pos {
            return dist;
        }

        for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = dx + pos.0;
            let new_y = dy + pos.1;
            if new_x < 0 || new_y < 0 {
                continue;
            }

            let new_pos = (new_x, new_y);
            if !visited.contains(&new_pos) && (open_spaces.contains(&new_pos) || space(new_pos, num)) {
                open_spaces.insert(new_pos);
                to_check.push_back((dist + 1, new_pos));
            }
        }
    }

    return -1;
}

fn part2(contents: String) -> i64 {
    let mut open_spaces: HashSet<(i64, i64)> = HashSet::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let num: i64 = contents.trim().parse().unwrap();

    let mut to_check: VecDeque<(i64, (i64, i64))> = VecDeque::from(vec![(0, (1, 1))]);
    while let Some((dist, pos)) = to_check.pop_front() {
        if visited.contains(&pos) || dist > 50 {
            continue;
        }
        visited.insert(pos);

        for (dx, dy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = dx + pos.0;
            let new_y = dy + pos.1;
            if new_x < 0 || new_y < 0 {
                continue;
            }

            let new_pos = (new_x, new_y);
            if !visited.contains(&new_pos) && (open_spaces.contains(&new_pos) || space(new_pos, num)) {
                open_spaces.insert(new_pos);
                to_check.push_back((dist + 1, new_pos));
            }
        }
    }

    return visited.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents = "10".to_string();

        assert_eq!(part1(contents, (7, 4)), 11);
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
    let day = "13".to_string();

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
        "\nPart 1:\nSteps to goal: {}\nRan in {:.5?}",
        part1(contents.clone(), (31, 39)),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLocations within 50 steps: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}