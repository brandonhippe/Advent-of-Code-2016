use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn disk_checksum(contents: String, length: usize) -> String {
    let mut chk = contents.trim().to_string();

    while chk.len() < length {
        let b = chk.chars().rev().map(|c| {
            match c {
                '1' => '0',
                '0' => '1',
                _ => panic!("Unknown char")
            }
        }).fold(String::new(), |mut s, c| {s.push(c); s});
        chk.push('0');
        chk.push_str(&b);
    }

    chk = chk[0..length].to_string();

    while chk.len() % 2 == 0 {
        chk = chk.chars().collect::<Vec<char>>().chunks(2).map(|cs| {
            if cs[0] == cs[1] {
                '1'
            } else {
                '0'
            }
        }).fold(String::new(), |mut s, c| {s.push(c); s});
    }

    return chk;
}

fn part1(contents: String, length: usize) -> String {
    return disk_checksum(contents, length);
}

fn part2(contents: String) -> String {
    return disk_checksum(contents, 35651584);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 20), "01100");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "16".to_string();

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
        "\nPart 1:\nChecksum: {}\nRan in {:.5?}",
        part1(contents.clone(), 272),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nChecksum: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}