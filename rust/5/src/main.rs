use relative_path::RelativePath;
use std::env;
use std::fs;
use std::collections::BinaryHeap;
use std::sync::mpsc::channel;
use std::thread::{self, available_parallelism};
use std::time::Instant;

fn part1(contents: String) -> String {
    let mut order: BinaryHeap<(usize, char)> = BinaryHeap::new();

    let num_cpus = usize::from(available_parallelism().unwrap()) - 1;
    let (tx, rx) = channel();
    let mut start_ix = 0;
    let inc_amt = 20_000;

    while order.len() < 8 {
        let threads: Vec<_> = (0..num_cpus).map(|n| {   
            let tx_send = tx.clone();
            let cloned_contents = contents.clone();
            thread::spawn(move || {
                let mut test_ix = start_ix + n;
                while test_ix < start_ix + inc_amt {
                    let hash = format!("{:x}", md5::compute(format!("{}{}", cloned_contents, test_ix)));
                    if hash.starts_with("00000") {
                        if tx_send.send((test_ix, hash.chars().nth(5).unwrap())).is_err() {
                            break;
                        }
                    }
                    test_ix += num_cpus;
                }

                test_ix
            })
        }).collect();
        let _last_ixs: Vec<usize> = threads.into_iter().map(|t| t.join().unwrap()).collect();

        while let Ok((test_ix, c)) = rx.try_recv() {
            order.push((test_ix, c));
        }

        start_ix += inc_amt;
    }

    return order.into_sorted_vec().iter().rev().take(8).rev().map(|(_, c)| *c).collect();
}

fn part2(contents: String) -> String {
    let mut pw: Vec<Vec<(usize, char)>> = vec![vec![]; 8];

    let threads = usize::from(available_parallelism().unwrap());
    let (tx, rx) = channel();

    for ix in 0..threads {
        let contents = contents.clone();

        let tx1 = tx.clone();

        std::thread::spawn(move || {
            let mut thread_ix = ix;
            let threads = threads;

            loop {
                let hash = format!("{:x}", md5::compute(format!("{}{}", contents, thread_ix)));
                if hash.starts_with("00000") {
                    if tx1
                        .send((
                            thread_ix,
                            hash.chars().nth(5).unwrap().to_digit(10),
                            hash.chars().nth(6).unwrap(),
                        ))
                        .is_err()
                    {
                        break;
                    }
                }

                thread_ix += threads;
            }
        });
    }

    while pw.iter().any(|v| v.len() < 1) {
        let (ix, val, c) = rx.recv().unwrap();
        if val.is_some() && val.unwrap() < 8 {
            pw[val.unwrap() as usize].push((ix, c));
        }
    }

    pw.iter_mut().for_each(|v| v.sort_by(|a, b| a.0.cmp(&b.0)));
    return pw.iter().map(|v| v[0].1).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents = "abc".to_string();

        assert_eq!(part1(contents), "18f47a30".to_string());
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2016".to_string();
    let day = "5".to_string();

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
        "\nPart 1:\nPassword: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPassword: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}