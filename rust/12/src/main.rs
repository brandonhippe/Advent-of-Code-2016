use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct State {
    pc: i64,
    registers: Vec<i64>,
}

impl State {
    fn run_instruction(&mut self, ins_str: &str) {
        let ins = ins_str.split_whitespace().next().unwrap();
        let ins_input = ins_str.replace(&ins, "");
        match ins {
            "cpy" => self.cpy(ins_input.trim()),
            "inc" => self.inc(ins_input.trim()),
            "dec" => self.dec(ins_input.trim()),
            "jnz" => self.jnz(ins_input.trim()),
            &_ => ()
        }
    }

    fn get_val(&self, val: &str) -> i64 {
        if val.parse::<i64>().is_ok() {
            val.parse::<i64>().unwrap()
        } else {
            self.registers[(val.chars().next().unwrap() as usize) - ('a' as usize)]
        }
    }

    fn cpy(&mut self, ins_str: &str) {
        let mut parts = ins_str.split_whitespace();
        let val = self.get_val(parts.next().unwrap());
        let reg = parts.next().unwrap();
        self.registers[(reg.chars().next().unwrap() as usize) - ('a' as usize)] = val;
        self.pc += 1
    }

    fn inc(&mut self, ins_str: &str) {
        self.registers[(ins_str.chars().next().unwrap() as usize) - ('a' as usize)] += 1;
        self.pc += 1;
    }

    fn dec(&mut self, ins_str: &str) {
        self.registers[(ins_str.chars().next().unwrap() as usize) - ('a' as usize)] -= 1;
        self.pc += 1;
    }

    fn jnz(&mut self, ins_str: &str) {
        let mut parts = ins_str.split_whitespace();
        let reg = self.get_val(parts.next().unwrap());
        let val = self.get_val(parts.next().unwrap());

        self.pc += if reg != 0 {
            val
        } else {
            1
        };
    }
}

impl Default for State {
    fn default() -> State {
        State {
            pc: 0,
            registers: vec![0; 4]
        }
    }
}

fn part1(contents: String) -> i64 {
    let instructions: Vec<&str> = contents.lines().collect::<Vec<_>>();
    let mut state = State { ..Default::default() };

    while 0 <= state.pc && state.pc < instructions.len() as i64 {
        state.run_instruction(instructions[state.pc as usize]);
    }

    return state.registers[0];
}

fn part2(contents: String) -> i64 {
    let instructions: Vec<&str> = contents.lines().collect::<Vec<_>>();
    let mut state = State { registers: vec![0, 0, 1, 0], ..Default::default() };

    while 0 <= state.pc && state.pc < instructions.len() as i64 {
        state.run_instruction(instructions[state.pc as usize]);
    }

    return state.registers[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 42);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "12".to_string();

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
        "\nPart 1:\nRegister a: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRegister a: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}