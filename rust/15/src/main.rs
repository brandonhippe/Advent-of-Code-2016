use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x, y) = gcd_extended(b % a, a);
    return (gcd, y - (b / a) * x, x);
}

fn rem(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

#[derive(Clone, Debug)]
struct Congruence {
    a: i64,
    n: i64,
    y: i64,
    z: i64,
}

impl Default for Congruence {
    fn default() -> Congruence {
        Congruence{
            a: 0,
            n: 0,
            y: 0,
            z: 0,
        }
    }
}

impl Congruence {
    fn new(cong_str: &str) -> Congruence {
        let int_re = Regex::new(r"\d+").unwrap();
        let ints: Vec<i64> = int_re.find_iter(cong_str).skip(1).step_by(2).map(|n| n.as_str().parse::<i64>().unwrap()).collect();

        Congruence {
            n: ints[0],
            a: ints[1],
            ..Default::default()
        }
    }

    fn calc_y(&mut self, big_n: i64) {
        self.y = big_n / self.n;
    }

    fn calc_z(&mut self) {
        self.z = rem(gcd_extended(self.y, self.n).1, self.n);
    }
}

fn pass_discs(contents: String) -> i64 {
    let mut congruences: Vec<Congruence> = contents.lines().map(|l| Congruence::new(l)).collect();
    let big_n: i64 = congruences.iter().map(|c| c.n).fold(1, |prod, n| prod * n);

    return big_n - rem(congruences.iter_mut().enumerate().map(|(ix, cong)| {
        cong.a += 1 + ix as i64;
        cong.calc_y(big_n);
        cong.calc_z();
        cong.a * cong.y * cong.z
    }).sum::<i64>(), big_n);
}

fn part1(contents: String) -> i64 {
    return pass_discs(contents);
}

fn part2(mut contents: String) -> i64 {
    contents = contents.trim().to_string();
    contents.push_str("\nDisc #69 has 11 positions; at time=0, it is at position 0.");
    return pass_discs(contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 5);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "15".to_string();

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
        "\nPart 1:\nTime to release: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTime to release: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}