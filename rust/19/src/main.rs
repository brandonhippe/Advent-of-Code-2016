use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let num_people: i64 = contents.trim().parse::<i64>().unwrap();
    let mut largest_power_2: i64 = 1;
    while largest_power_2 < num_people {
        largest_power_2 <<= 1;
    }
    largest_power_2 >>= 1;

    let mut victor: i64 = 1;
    let mut inc: i64 = 2;
    for _ in largest_power_2..num_people {
        victor += inc;
        if victor > num_people {
            victor %= num_people;
            victor += 1;
            inc *= 2;
        }
    }
    return victor;
}

fn part2(contents: String) -> i64 {
    let num_people: i64 = contents.trim().parse::<i64>().unwrap();
    let mut largest_mult_3: i64 = 1;
    while largest_mult_3 * 3 < num_people {
        largest_mult_3 *= 3;
    }

    if num_people <= 2 * largest_mult_3 {
        num_people - largest_mult_3
    } else {
        largest_mult_3 + 2 * (num_people - (2 * largest_mult_3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        assert_eq!(part1("5".to_string()), 3);
    }

    #[test]
    fn p2_test() {
        assert_eq!(part2("5".to_string()), 2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "19".to_string();

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
        "\nPart 1:\nWinner: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nWinner: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}