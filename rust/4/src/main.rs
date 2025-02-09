use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn part1(contents: String) -> i64 {
    return contents.lines().map(|line| {
        let l = line.split("[").next().unwrap();
        let letter_occs: HashMap<char, i64> = l.chars()
            .fold(HashMap::new(), |mut acc, c| {
                if c.is_alphabetic() {
                    *acc.entry(c).or_insert(0) += 1;
                }
                acc
            });

        let mut top_five: Vec<(char, i64)> = letter_occs.iter().map(|(k, v)| (*k, *v)).collect();
        top_five.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            return b.1.cmp(&a.1);
        });

        let checksum = line.split("[").nth(1).unwrap().chars().filter(|c| c.is_alphabetic()).collect::<String>();
        let calc_checksum = top_five.iter().take(5).map(|(c, _)| c).collect::<String>();

        return if checksum == calc_checksum { l.split("-").last().unwrap().parse::<i64>().unwrap() } else { 0 };
    }).sum();
}

fn part2(contents: String) -> i64 {
    for line in contents.lines() {
        let encrypted_name = line.split("[").next().unwrap().chars().filter(|c| c.is_alphabetic() || *c == '-').collect::<String>();

        let sector_id = line.split("[").next().unwrap().split("-").last().unwrap().parse::<i64>().unwrap();

        let decrypted_name = encrypted_name.chars().map(|c| {
            if c == '-' {
                return ' ';
            }
            return (((c as i64 - 'a' as i64 + sector_id) % 26 + 26) % 26 + 'a' as i64) as u8 as char;
        }).collect::<String>();

        if decrypted_name.contains("north") {
            return sector_id;
        }
    }

    return -1;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2016".to_string();
    let day = "4".to_string();

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
        "\nPart 1:\nSum of sector IDs: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSector ID of north pole storage: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}