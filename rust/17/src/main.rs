use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::VecDeque;
use std::iter::zip;

fn get_directions(seed: &str, path: &str) -> Vec<(char, (i64, i64))> {
    let char_dirs: Vec<(char, (i64, i64))> = vec![('U', (0, -1)), ('D', (0, 1)), ('L', (-1, 0)), ('R', (1, 0))];
    let hash = format!("{:x}", md5::compute(format!("{}{}", seed, path)));

    zip(char_dirs.iter(), hash[0..4].chars()).filter_map(|(dir_info, c)| {
        if c as u32 > 'a' as u32 && c as u32 <= 'f' as u32 {
            Some(*dir_info)
        } else {
            None
        }
    }).collect()
}

fn part1(contents: String) -> String {
    let seed = contents.trim();
    let goal: (i64, i64) = (3, 3);
    let mut open_list: VecDeque<(String, (i64, i64))> = VecDeque::from([(String::from(""), (0, 0))]);

    while let Some((path_str, pos)) = open_list.pop_front() {
        if pos == goal {
            return path_str.to_string();
        }

        let (x, y) = pos;
        for (append_c, (x_off, y_off)) in get_directions(seed, &path_str) {
            let mut new_str = path_str.to_string();
            new_str.push(append_c);
            open_list.push_back((new_str, (x + x_off, y + y_off)))
        }
    }

    return "".to_string();
}

fn part2(contents: String) -> i64 {
    let seed = contents.trim();
    let goal: (i64, i64) = (3, 3);
    let mut open_list: VecDeque<(String, (i64, i64))> = VecDeque::from([(String::from(""), (0, 0))]);

    let mut max_len: i64 = 0;
    while let Some((path_str, pos)) = open_list.pop_front() {
        if pos == goal {
            max_len = max_len.max(path_str.len() as i64);
            continue;
        }

        let (x, y) = pos;
        for (append_c, (x_off, y_off)) in get_directions(seed, &path_str) {
            let new_pos = (x + x_off, y + y_off);
            if new_pos.0 < 0 || new_pos.0 > 3 || new_pos.1 < 0 || new_pos.1 > 3 {
                continue;
            }
            let mut new_str = path_str.to_string();
            new_str.push(append_c);
            open_list.push_back((new_str, new_pos))
        }
    }

    return max_len;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        assert_eq!(part1("ihgpwlah".to_string()), "DDRRRD");
        assert_eq!(part1("kglvqrro".to_string()), "DDUDRLRRUDRD");
    }

    #[test]
    fn p2_test() {
        assert_eq!(part2("ihgpwlah".to_string()), 370);
        assert_eq!(part2("kglvqrro".to_string()), 492);
        assert_eq!(part2("ulqzkmiv".to_string()), 830);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "17".to_string();

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
        "\nPart 1:\nShortest path: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLongest path length: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}