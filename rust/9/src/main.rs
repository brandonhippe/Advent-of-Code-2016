use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;

fn decompress_len(string: &str, check_inside: bool) -> i64 {
    let re = Regex::new(r"\((?P<count>\d+)x(?P<repeat>\d+)\)").unwrap();
    let mut total_chars: i64 = 0;
    let mut char_ix: i64 = 0;

    for caps in re.captures_iter(string) {
        let group = caps.get(0).unwrap();
        let count: i64 = caps["count"].parse::<i64>().unwrap();
        let repeat: i64 = caps["repeat"].parse::<i64>().unwrap();
        if (group.start() as i64) < char_ix {
            continue;
        }
    
        if char_ix < group.start() as i64 {
            total_chars += group.start() as i64 - char_ix;
        }
        
        let char_count = if check_inside {
            let substring = &string[group.end()..group.end() + count as usize];
            decompress_len(substring, check_inside)
        } else {
            count
        };
        
        total_chars += char_count * repeat;
        char_ix = group.end() as i64 + count;
    }
    
    if char_ix < string.len() as i64 {
        total_chars += string.len() as i64 - char_ix;
    }

    return total_chars;
}

fn part1(contents: String) -> i64 {
    return contents.lines().map(|line| decompress_len(line, false)).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    return contents.lines().map(|line| decompress_len(line, true)).sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 57);
    }

    #[test]
    fn p2_test() {
        assert_eq!(part2("(3x3)XYZ".to_string()), 9);
        assert_eq!(part2("X(8x2)(3x3)ABCY".to_string()), 20);
        assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A".to_string()), 241920);
        assert_eq!(part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string()), 445);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "9".to_string();

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
        "\nPart 1:\nDecompressed length: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nDecompressed length: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}