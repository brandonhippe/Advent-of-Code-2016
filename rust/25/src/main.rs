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
    fn run(&mut self, test_len: usize) -> Vec<i64> {
        let mut out_vec: Vec<i64> = Vec::new();
        while out_vec.len() < test_len && 0 <= self.pc && (self.pc as usize) < self.instructions.len() {
            if let Some(output) = self.execute_instruction() {
                out_vec.push(output);
            }
        }
        
        return out_vec;
    }
    
    fn execute_instruction(&mut self) -> Option<i64> {
        let ins = self.instructions[self.pc as usize];
        let ins_words: Vec<&str> = ins.split_whitespace().collect();
        
        match ins.split_whitespace().next().unwrap() {
            "cpy" => {
                if let Some(reg) = self.get_reg(&ins_words, 2) {
                    if let Some(v) = self.get_reg_val(&ins_words, 1) {
                        self.registers.insert(reg, v);
                    }
                }

                self.pc += 1;
            },
            "inc" => {
                if let Some(reg) = self.get_reg(&ins_words, 1) {
                    *self.registers.entry(reg).or_insert(0) += 1;
                }

                self.pc += 1;
            },
            "dec" => {
                if let Some(reg) = self.get_reg(&ins_words, 1) {
                    *self.registers.entry(reg).or_insert(0) -= 1;
                }

                self.pc += 1;
            },
            "jnz" => {
                if let Some(check) = self.get_reg_val(&ins_words, 1) {
                    if check == 0 {
                        self.pc += 1;
                    } else if let Some(v) = self.get_reg_val(&ins_words, 2) {
                        self.pc += v;
                    } else {
                        self.pc += 1;
                    }
                } else {
                    self.pc += 1;
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
                
                self.pc += 1;
            },
            "out" => {
                self.pc += 1;
                if let Some(v) = self.get_reg_val(&ins_words, 1) {
                    return Some(v);
                }
            }
            _ => panic!("Unknown instruction!")
        }

        return None;
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
    let test_len = 10;
    let mut test_a: i64 = 0;

    loop {
        let outputs = Program {
            instructions: contents.lines().map(|l| l.trim()).collect(),
            registers: HashMap::from([('a', test_a)]),
            ..Default::default()
        }.run(test_len);

        if outputs.windows(2).filter(|w| w[0] == w[1]).count() == 0 {
            return test_a;
        }

        test_a += 1;
    }
}

fn part2(contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2016".to_string();
    let day = "25".to_string();

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
        "\nPart 1:\nValue to initialize clock: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}