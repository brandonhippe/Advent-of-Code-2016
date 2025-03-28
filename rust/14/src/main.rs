use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BinaryHeap};
use std::thread::{self, available_parallelism};
use std::sync::mpsc;
use std::ops::Bound::{Included, Excluded};

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
    let (tx, rx) = mpsc::channel();
    let num_cpus = available_parallelism().unwrap().get() - 1;
    
    let mut small_matches: BTreeMap<usize, char> = BTreeMap::new();
    let mut big_matches: BTreeMap<usize, Vec<char>> = BTreeMap::new();
    let mut found_keys: HashSet<usize> = HashSet::new();
    let mut start_ix = 0;
    let mut thread_recv: Vec<BinaryHeap<usize>> = vec![BinaryHeap::new(); num_cpus];
    let inc_amt = 1_000;

    loop {
        let threads: Vec<_> = (0..num_cpus).map(|n| {   
            let tx_send = tx.clone();
            let cloned_contents = contents.clone();
            thread::spawn(move || {
                let mut test_ix = start_ix + n;
                while test_ix < start_ix + inc_amt {
                    let hash = (0..2016).fold(format!("{:x}", md5::compute(format!("{}{}", cloned_contents.trim(), test_ix))), |hash, _| format!("{:x}", md5::compute(hash)));
                    for (m_len, matches) in consec_match(hash).iter().filter(|(&k, _v)| k == 3 || k == 5) {
                        tx_send.send((test_ix, m_len.clone(), matches.clone())).unwrap(); 
                    }
                    
                    test_ix += num_cpus;
                }

                test_ix
            })
        }).collect();
        let _last_ixs: Vec<usize> = threads.into_iter().map(|t| t.join().unwrap()).collect();

        while let Ok((test_ix, m_len, matches)) = rx.try_recv() {
            match m_len {
                3 => {
                    let thread_ix = (test_ix % inc_amt) % num_cpus;
                    thread_recv[thread_ix].push(test_ix);
                    let first_char = matches.keys().map(|k| *k).min_by_key(|k| matches.get(&k)).unwrap();
                    small_matches.insert(test_ix, first_char);
                },
                5 => {
                    for match_c in matches.into_keys() {
                        big_matches.entry(test_ix).or_insert(Vec::new()).push(match_c);
                    }
                },
                _ => panic!("Unknown match length: {}", m_len)
            }
        }

        let mut new_found: HashSet<usize> = HashSet::new();
        for (&sm, c) in small_matches.iter().filter(|(sm, _c)| !found_keys.contains(sm)) {
            if big_matches.range((Excluded(sm), Included(sm + 1000))).any(|(_, v)| v.contains(c)) {
                new_found.insert(sm);
            }
        }
        found_keys = found_keys.union(&new_found).map(|k| *k).collect();

        if found_keys.len() >= 64 && thread_recv.iter().all(|ixs| {
            let latest: usize = *ixs.peek().unwrap();
            if let Some(&last_key) = found_keys.iter().max() {
                latest > last_key
            } else {
                false
            }
        }) {
            break;
        }

        start_ix += inc_amt;
    }
    
    let mut key_order: Vec<usize> = found_keys.iter().map(|v| *v).collect();
    key_order.sort();
    return key_order[63] as i64;
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