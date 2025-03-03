use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap, VecDeque};

fn fewest_steps(curr_node: char, neighbors: &HashMap<char, HashMap<char, i64>>, visited: HashSet<char>, p2: bool) -> i64 {
    if visited.len() == neighbors.len() {
        return if !p2 {
            0
        } else if let Some(v) = neighbors.get(&curr_node).unwrap().get(&'0') {
            *v
        } else {
            i64::MAX >> 2
        }
    }

    let mut min_steps: i64 = i64::MAX >> 2;
    for (nc, add_steps) in neighbors.get(&curr_node).unwrap().iter().filter(|(c, _)| !visited.contains(c)) {
        let new_visited: HashSet<char> = visited.union(&HashSet::from([*nc])).cloned().collect();
        min_steps = min_steps.min(fewest_steps(*nc, neighbors, new_visited, p2) + add_steps);
    }

    return min_steps;
}

fn part1(contents: String) -> i64 {
    let mut free: HashSet<(i64, i64)> = HashSet::new();
    let mut pois: HashMap<(i64, i64), char> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => (),
                _ => {
                    free.insert((x as i64, y as i64));
                    if c != '.' {
                        pois.insert((x as i64, y as i64), c);
                    }
                }
            }
        }
    }

    let mut neighbors: HashMap<char, HashMap<char, i64>> = HashMap::new();
    for (pos, start_c) in pois.iter() {
        let mut to_check: VecDeque<((i64, i64), i64)> = VecDeque::from([(*pos, 0)]);
        let mut visited: HashSet<(i64, i64)> = HashSet::new();

        while let Some((p, steps)) = to_check.pop_front() {
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);

            if p != *pos && pois.contains_key(&p) {
                let n = *pois.get(&p).unwrap();
                neighbors.entry(*start_c).or_insert(HashMap::new()).insert(n, steps);
            }

            for (dx, dy) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                let new_pos = (p.0 + dx, p.1 + dy);
                if !visited.contains(&new_pos) && free.contains(&new_pos) {
                    to_check.push_back((new_pos, steps + 1));
                }
            }
        }
    }

    return fewest_steps('0', &neighbors, HashSet::from(['0']), false);
}

fn part2(contents: String) -> i64 {
    let mut free: HashSet<(i64, i64)> = HashSet::new();
    let mut pois: HashMap<(i64, i64), char> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => (),
                _ => {
                    free.insert((x as i64, y as i64));
                    if c != '.' {
                        pois.insert((x as i64, y as i64), c);
                    }
                }
            }
        }
    }

    let mut neighbors: HashMap<char, HashMap<char, i64>> = HashMap::new();
    for (pos, start_c) in pois.iter() {
        let mut to_check: VecDeque<((i64, i64), i64)> = VecDeque::from([(*pos, 0)]);
        let mut visited: HashSet<(i64, i64)> = HashSet::new();

        while let Some((p, steps)) = to_check.pop_front() {
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);

            if p != *pos && pois.contains_key(&p) {
                let n = *pois.get(&p).unwrap();
                neighbors.entry(*start_c).or_insert(HashMap::new()).insert(n, steps);
            }

            for (dx, dy) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                let new_pos = (p.0 + dx, p.1 + dy);
                if !visited.contains(&new_pos) && free.contains(&new_pos) {
                    to_check.push_back((new_pos, steps + 1));
                }
            }
        }
    }

    return fewest_steps('0', &neighbors, HashSet::from(['0']), true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 14);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "24".to_string();

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
        "\nPart 1:\nFewest steps: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nFewest steps returning to start: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}