use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::{HashMap,VecDeque};


fn sort_chips(bins: &mut HashMap<String, Vec<i64>>, bots: HashMap<String, (String, String)>, compare: Option<(i64, i64)>) -> Option<i64> {
    let int_re: Regex = Regex::new(r"(?P<output>\w+ (?P<num>\d+))").unwrap();
    let mut to_give_bots = VecDeque::from_iter(bins.iter().filter(|(_, v)| v.len() == 2).map(|(k, _)| k.clone()));
    while to_give_bots.len() > 0 {
        let bot = to_give_bots.pop_front().unwrap();
        let mut chips = bins.remove(&bot).unwrap();
    
        chips.sort();
        let low = chips[0];
        let high = chips[1];
    
        if let Some((compare_low, compare_high)) = compare {
            if compare_low == low && compare_high == high {
                return Some(int_re.captures_iter(&bot).map(|c| c["num"].parse::<i64>().unwrap()).next().unwrap());
            }
        }
    
        let (low_bin, high_bin) = bots[&bot].clone();
        bins.entry(low_bin.clone()).or_insert(Vec::new()).push(low);
        if low_bin.split_whitespace().next().unwrap() == "bot" && bins.get(&low_bin).unwrap().len() == 2 {
            to_give_bots.push_back(low_bin);
        }
        
        bins.entry(high_bin.clone()).or_insert(Vec::new()).push(high);
        if high_bin.split_whitespace().next().unwrap() == "bot" && bins.get(&high_bin).unwrap().len() == 2 {
            to_give_bots.push_back(high_bin);
        }
    }
    
    return None;
}

fn part1(contents: String, compare: (i64, i64)) -> i64 {
    let int_re: Regex = Regex::new(r"(?P<output>\w+ (?P<num>\d+))").unwrap();
    let mut bins: HashMap<String, Vec<i64>> = HashMap::new();
    let mut bots: HashMap<String, (String, String)> = HashMap::new();

    for line in contents.lines() {
        let cap_iter = int_re.captures_iter(line);
        let caps: Vec<_> = cap_iter.collect();
        if caps.len() == 2 {
            let value: &i64 = &caps[0]["num"].parse().unwrap();
            let bot = &caps[1]["output"];

            bins.entry(bot.to_string()).or_insert(Vec::new()).push(*value);
        } else {
            let values: Vec<String> = caps.into_iter().map(|c| c["output"].to_string()).collect();
            let bot = &values[0];
            let low = &values[1];
            let high = &values[2];

            bots.insert(bot.clone(), (low.clone(), high.clone()));
        }
    }

    return sort_chips(&mut bins, bots, Some(compare)).expect("No compare bot found!");
}

fn part2(contents: String) -> i64 {
    let int_re: Regex = Regex::new(r"(?P<output>\w+ (?P<num>\d+))").unwrap();
    let mut bins: HashMap<String, Vec<i64>> = HashMap::new();
    let mut bots: HashMap<String, (String, String)> = HashMap::new();

    for line in contents.lines() {
        let cap_iter = int_re.captures_iter(line);
        let caps: Vec<_> = cap_iter.collect();
        if caps.len() == 2 {
            let value: &i64 = &caps[0]["num"].parse().unwrap();
            let bot = &caps[1]["output"];

            bins.entry(bot.to_string()).or_insert(Vec::new()).push(*value);
        } else {
            let values: Vec<String> = caps.into_iter().map(|c| c["output"].to_string()).collect();
            let bot = &values[0];
            let low = &values[1];
            let high = &values[2];

            bots.insert(bot.clone(), (low.clone(), high.clone()));
        }
    }
    sort_chips(&mut bins, bots, None);

    return (0..=2)
        .map(|v| bins[&format!("output {}", v)][0])
        .product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, (2, 5)), 2);
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
    let day = "10".to_string();

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
        "\nPart 1:\nBot that compares 61 and 17: {}\nRan in {:.5?}",
        part1(contents.clone(), (17, 61)),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nProduct of chips in outputs 0, 1, and 2: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}