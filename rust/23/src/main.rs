use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Program<'a> {
    instructions: Vec<&'a str>,
    pc: i64,
    registers: HashMap<char, i64>,
}

impl Default for Program<'_> {
    fn default() -> Program<'static> {
        Program {
            instructions: vec![],
            pc: 0,
            registers: HashMap::new(),
        }
    }
}

impl Program<'_> {
    fn run_p1(&mut self) -> i64 {
        while 0 <= self.pc && (self.pc as usize) < self.instructions.len() {
            self.execute_instruction();
        }
        
        return *self.registers.get(&'a').unwrap_or(&0);
    }

    fn run_p2(&mut self) -> i64 {
        while 0 <= self.pc && (self.pc as usize) < self.instructions.len() {
            if self.pc == 4 {
                *self.registers.entry('a').or_insert(0) += self.registers.get(&'b').unwrap_or(&0) * self.registers.get(&'d').unwrap_or(&0);
                *self.registers.entry('b').or_insert(0) -= 1;
                self.registers.insert('c', 2 * self.registers.get(&'b').unwrap_or(&0));
                self.registers.insert('d', 0);
                self.pc = 16;
            } else {
                self.execute_instruction();
            }
        }
        
        return *self.registers.get(&'a').unwrap_or(&0);
    }
    
    fn execute_instruction(&mut self) {
        let ins = self.instructions[self.pc as usize];
        let ins_words: Vec<&str> = ins.split_whitespace().collect();
        
        self.pc += match ins.split_whitespace().next().unwrap() {
            "cpy" => {
                if let Some(reg) = self.get_reg(&ins_words, 2) {
                    if let Some(v) = self.get_reg_val(&ins_words, 1) {
                        self.registers.insert(reg, v);
                    }
                }

                1
            },
            "inc" => {
                if let Some(reg) = self.get_reg(&ins_words, 1) {
                    *self.registers.entry(reg).or_insert(0) += 1;
                }

                1
            },
            "dec" => {
                if let Some(reg) = self.get_reg(&ins_words, 1) {
                    *self.registers.entry(reg).or_insert(0) -= 1;
                }

                1
            },
            "jnz" => {
                if let Some(check) = self.get_reg_val(&ins_words, 1) {
                    if check == 0 {
                        1
                    } else if let Some(v) = self.get_reg_val(&ins_words, 2) {
                        v
                    } else {
                        1
                    }
                } else {
                    1
                }
            },
            "tgl" => {
                if let Some(v) = self.get_reg_val(&ins_words, 1) {
                    let ins_ix = self.pc + v;
                    if ins_ix >= 0 && ins_ix < self.instructions.len() as i64 {
                        let check = self.instructions[ins_ix as usize];
                        let mut check_words: Vec<&str> = check.split_whitespace().collect();
                        
                        check_words[0] = match (check_words.len(), check_words[0]) {
                            (2, "inc") => "dec",
                            (2, _) => "inc",
                            (3, "jnz") => "cpy",
                            (3, _) => "jnz",
                            _ => panic!("Unknown instruction")
                        };
                        
                        let new_instruction = check_words.join(" ");
                        self.instructions[ins_ix as usize] = Box::leak(new_instruction.into_boxed_str());
                    }
                }
                
                1
            },
            _ => panic!("Unknown instruction!")
        };
    }
    
    fn get_reg(&self, ins_words: &Vec<&str>, reg_ix: usize) -> Option<char> {
        let ok_chars = "abcd".to_string();
        let reg: char = ins_words[reg_ix].chars().next().unwrap();

        if ok_chars.contains(reg) {
            Some(reg)
        } else {
            None
        }
    }

    fn get_reg_val(&mut self, ins_words: &Vec<&str>, reg_ix: usize) -> Option<i64> {
        let ok_chars = "abcd".to_string();
        let reg: char = ins_words[reg_ix].chars().next().unwrap();

        if ok_chars.contains(reg) {
            Some(*self.registers.entry(reg).or_insert(0))
        } else if let Ok(v) = ins_words[reg_ix].parse::<i64>() {
            Some(v)
        } else {
            None
        }
    }
}

fn part1(contents: String) -> i64 {
    let mut program = Program {
        instructions: contents.lines().map(|l| l.trim()).collect(),
        registers: HashMap::from([('a', 7)]),
        ..Default::default()
    };
    return program.run_p1();
}

fn part2(contents: String) -> i64 {
    let mut program = Program {
        instructions: contents.lines().map(|l| l.trim()).collect(),
        registers: HashMap::from([('a', 12)]),
        ..Default::default()
    };
    return program.run_p2();}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        let mut program = Program {
            instructions: contents.lines().map(|l| l.trim()).collect(),
            ..Default::default()
        };
        assert_eq!(program.run_p1(), 3);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "23".to_string();

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
        "\nPart 1:\nSend to safe: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSend to safe: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}