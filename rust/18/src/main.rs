use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn tiles(contents: String, num_lines: usize) -> i64 {
    let mut p_line: String = contents.trim().to_string();
    let mut safe_count: i64 = p_line.chars().filter(|c| *c == '.').count() as i64;
    
    for _ in 1..num_lines {
        p_line = format!(".{}.", p_line).chars().collect::<Vec<_>>().windows(3).map(|l| {
            let window: &str = &l.iter().fold("".to_string(), |mut s, c| {s.push(*c); s});
            
            match window {
                "^^." => '^',
                ".^^" => '^',
                "..^" => '^',
                "^.." => '^',
                _ => {
                    safe_count += 1;
                    '.'
                }
            }
        }).fold("".to_string(), |mut s, c| {s.push(c); s});
    }

    return safe_count;
}

fn part1(contents: String, num_lines: usize) -> i64 {
    return tiles(contents, num_lines);
}

fn part2(contents: String) -> i64 {
    return tiles(contents, 400_000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 10), 38);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "18".to_string();

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
        "\nPart 1:\nSafe tiles: {}\nRan in {:.5?}",
        part1(contents.clone(), 40),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSafe tiles: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}