use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{VecDeque, HashMap, HashSet};

fn consec_match(hash: String) -> HashMap<i64, HashMap<char, usize>> {
    let mut p_char = hash.chars().next().unwrap();
    let mut start: usize = 0;
    let mut consec: i64 = 0;
    let mut match_lens: HashMap<i64, HashMap<char, usize>> = HashMap::new();

    for (ix, c) in hash.chars().enumerate() {
        if c != p_char {
            p_char = c;
            start = ix;
            consec = 0;
        }
        consec += 1;
        match_lens.entry(consec).or_insert(HashMap::new()).insert(c, start);
    }
    
    return match_lens;
}

fn part1(contents: String) -> i64 {
    let mut pos_keys: VecDeque<(i64, char)> = VecDeque::new();
    let mut found_keys: HashSet<i64> = HashSet::new();

    let mut test_ix: i64 = 0;
    while found_keys.len() < 64 || pos_keys.len() > 0 {
        let hash = format!("{:x}", md5::compute(format!("{}{}", contents.trim(), test_ix)));
        let hash_matches = consec_match(hash);

        if let Some(big_matches) = hash_matches.get(&5) {
            for big_match in big_matches.clone().into_keys() {
                for ix in pos_keys.clone().iter().filter(|(_, s)| *s == big_match) {
                    found_keys.insert(ix.0);
                }
            }
        }

        if found_keys.len() < 64 {
            if let Some(small_matches) = hash_matches.get(&3) {
                let mut pos_key: Vec<_> = small_matches.iter().collect();
                pos_key.sort_by(|a, b| a.1.cmp(b.1));
                pos_keys.push_back((test_ix, *pos_key.first().unwrap().0));
            }
        }

        while pos_keys.len() > 0 && pos_keys[0].0 + 1000 <= test_ix {
            pos_keys.pop_front();
        }
        test_ix += 1;
    }
    
    let mut key_order: Vec<i64> = found_keys.iter().map(|v| *v).collect();
    key_order.sort();
    return key_order[63];
}

fn part2(contents: String) -> i64 {
    let mut pos_keys: VecDeque<(i64, char)> = VecDeque::new();
    let mut found_keys: HashSet<i64> = HashSet::new();

    let mut test_ix: i64 = 0;
    while found_keys.len() < 64 || pos_keys.len() > 0 {
        let mut hash = format!("{:x}", md5::compute(format!("{}{}", contents.trim(), test_ix)));
        for _ in 0..2016 {
            hash = format!("{:x}", md5::compute(hash));
        }
        let hash_matches = consec_match(hash);

        if let Some(big_matches) = hash_matches.get(&5) {
            for big_match in big_matches.clone().into_keys() {
                for ix in pos_keys.clone().iter().filter(|(_, s)| *s == big_match) {
                    found_keys.insert(ix.0);
                }
            }
        }

        if found_keys.len() < 64 {
            if let Some(small_matches) = hash_matches.get(&3) {
                let mut pos_key: Vec<_> = small_matches.iter().collect();
                pos_key.sort_by(|a, b| a.1.cmp(b.1));
                pos_keys.push_back((test_ix, *pos_key.first().unwrap().0));
            }
        }

        while pos_keys.len() > 0 && pos_keys[0].0 + 1000 <= test_ix {
            pos_keys.pop_front();
        }
        test_ix += 1;
    }
    
    let mut key_order: Vec<i64> = found_keys.iter().map(|v| *v).collect();
    key_order.sort();
    return key_order[63];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents = "abc".to_string();

        assert_eq!(part1(contents), 22728);
    }

    #[test]
    fn p2_test() {
        let contents = "abc".to_string();

        assert_eq!(part2(contents), 22551);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "14".to_string();

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
        "\nPart 1:\n64th key index: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n64th key index: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}