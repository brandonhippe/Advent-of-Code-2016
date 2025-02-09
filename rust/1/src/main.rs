use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return manhat_dist(
        contents
            .split(", ")
            .fold(((0, 0), (0, 1)), |acc, ins| follow_dir(acc.0, acc.1, ins))
            .0,
    );
}

fn part2(contents: String) -> i64 {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut pos = (0, 0);
    let mut facing = (0, 1);

    let ins: Vec<&str> = contents.split(", ").collect();
    let mut i = 0;

    loop {
        let (new_pos, new_facing) = follow_dir(pos, facing, ins[i]);

        for y in pos.1.min(new_pos.1)..=pos.1.max(new_pos.1) {
            for x in pos.0.min(new_pos.0)..=pos.0.max(new_pos.0) {
                if visited.contains(&(x, y)) {
                    return manhat_dist((x, y));
                }

                visited.insert((x, y));
            }
        }


        visited.remove(&new_pos);
        pos = new_pos;
        facing = new_facing;

        i += 1;
        i %= ins.len();
    }
}

fn follow_dir(pos: (i64, i64), facing: (i64, i64), ins: &str) -> ((i64, i64), (i64, i64)) {
    let new_dir = match ins.chars().nth(0).unwrap() {
        'R' => (facing.1, -facing.0),
        'L' => (-facing.1, facing.0),
        _ => panic!("Invalid direction"),
    };
    let new_pos = (
        pos.0 + new_dir.0 * ins[1..].parse::<i64>().unwrap(),
        pos.1 + new_dir.1 * ins[1..].parse::<i64>().unwrap(),
    );

    return (new_pos, new_dir);
}

fn manhat_dist(pos: (i64, i64)) -> i64 {
    return pos.0.abs() + pos.1.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let mut contents = "R2, L3".to_string();
        assert_eq!(part1(contents), 5);
        contents = "R2, R2, R2".to_string();
        assert_eq!(part1(contents), 2);
        contents = "R5, L5, R5, R3".to_string();
        assert_eq!(part1(contents), 12);
    }

    #[test]
    fn p2_test() {
        let contents = "R8, R4, R4, R8".to_string();
        assert_eq!(part2(contents), 4);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2016".to_string();
    let day = "1".to_string();

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
        "\nPart 1:\nDistance: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nDistance to first position visited twice: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}