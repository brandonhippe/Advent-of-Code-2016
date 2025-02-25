use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut ranges: Vec<(u32, u32)> = contents.lines().map(|l| {
        let mut parts = l.split('-').map(|c| c.parse::<u32>().unwrap());
        (parts.next().unwrap(), parts.next().unwrap())
    }).collect();

    ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    let mut max_end: u32 = 0;

    for r in ranges.windows(2) {
        max_end = max_end.max(r[0].1);
        if max_end < r[1].0 - 1 {
            return max_end as i64 + 1;
        }
    }
    return -1;
}

fn part2(contents: String, max_val: u32) -> i64 {
    let mut ranges: Vec<(u32, u32)> = contents.lines().map(|l| {
        let mut parts = l.split('-').map(|c| c.parse::<u32>().unwrap());
        (parts.next().unwrap(), parts.next().unwrap())
    }).collect();

    ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    let mut max_end: u32 = 0;
    let mut allowed: i64 = 0;

    for r in ranges.windows(2) {
        max_end = max_end.max(r[0].1);
        if max_end < r[1].0 - 1 {
            allowed += (r[1].0 - max_end) as i64 - 1;
        }
    }
    
    max_end = max_end.max(ranges.iter().last().unwrap().1);
    return allowed + (max_val - max_end) as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 3);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 9), 2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "20".to_string();

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
        "\nPart 1:\nSmallest allowed IP: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber of allowed IPs: {}\nRan in {:.5?}",
        part2(contents.clone(), u32::MAX),
        part2_timer.elapsed()
    );
}