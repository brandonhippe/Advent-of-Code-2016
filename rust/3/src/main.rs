use regex::Regex;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let int_re: Regex = Regex::new(r"\d+").unwrap();
    return contents
        .lines()
        .map(|line| {
            let mut nums: Vec<i64> = int_re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect();
            nums.sort();
            return (nums[0] + nums[1] > nums[2]) as i64;
        })
        .sum();
}

fn part2(contents: String) -> i64 {
    let int_re: Regex = Regex::new(r"\d+").unwrap();
    let mut nums: Vec<i64> = vec![0; contents.lines().count() * 3];
    for (i, line) in contents.lines().enumerate() {
        for (ix, n) in int_re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .enumerate()
        {
            nums[ix * 3 + i % 3 + (i / 3) * 9] = n;
        }
    }

    return nums

        .chunks(3)
        .map(|chunk| {
            let mut nums: Vec<i64> = chunk.iter().map(|&n| n).collect();
            nums.sort();
            return (nums[0] + nums[1] > nums[2]) as i64;
        })
        .sum();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2016".to_string();
    let day = "3".to_string();

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
        "\nPart 1:\nValid triangles: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nValid Triangles: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}