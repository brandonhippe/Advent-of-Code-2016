use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;

fn steps(counts: &mut Vec<i64>) -> i64 {
    let mut num_steps: i64 = 0;
    while *counts.iter().last().unwrap() != counts.clone().iter().sum::<i64>() {
        for (i, n) in counts.clone().iter().enumerate().filter(|(_, n)| **n != 0) {
            counts[i] = 0;
            counts[i + 1] += n;
            num_steps += 2 * (n-1) - 1;
            break;
        }
    }

    return num_steps;
}

fn part1(contents: String) -> i64 {
    let comp_regex: Regex = Regex::new(r"(?P<element>\w+)(?: |-)(?P<comp>compatible microchip|generator)").unwrap();
    let mut counts: Vec<i64> = vec![];

    for (ix, line) in contents.lines().enumerate() {
        if ix == counts.len() {
            counts.push(0);
        }
        counts[ix] += comp_regex.captures_iter(line).count() as i64;
    }

    return steps(&mut counts);
}

fn part2(contents: String) -> i64 {
    let comp_regex: Regex = Regex::new(r"(?P<element>\w+)(?: |-)(?P<comp>compatible microchip|generator)").unwrap();
    let mut counts: Vec<i64> = vec![4];

    for (ix, line) in contents.lines().enumerate() {
        if ix == counts.len() {
            counts.push(0);
        }
        counts[ix] += comp_regex.captures_iter(line).count() as i64;
    }

    return steps(&mut counts);
}

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "11".to_string();

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
        "\nPart 1:\nNumber of steps: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber of steps: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}