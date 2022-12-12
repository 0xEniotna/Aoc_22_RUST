use eyre::Result;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<()> {
    // question1()?;
    question2()?;

    Ok(())
}

fn question1() -> Result<()> {
    let lines = read_lines("inputs/input10.txt")?;
    // let lines = read_lines("inputs/test.txt")?;

    let mut res = 0;
    let mut instruction = vec!["".to_string()];
    let mut cycle = 1;
    let mut register: i32 = 1;

    for line in lines {
        let line = line?;
        instruction = line.split_whitespace().map(|s| s.to_string()).collect();

        match instruction.len() {
            1 => {
                if cycle == 20 {
                    res += cycle * register;
                }
                if cycle > 20 && (cycle - 20) % 40 == 0 {
                    res += cycle * register;
                }
                cycle += 1;
            }
            2 => {
                for _ in 0..2 {
                    if cycle == 20 {
                        res += cycle * register;
                    }
                    if cycle > 20 && (cycle - 20) % 40 == 0 {
                        res += cycle * register;
                    }

                    cycle += 1;
                }
                register += instruction[1].parse::<i32>().unwrap();
            }
            _ => println!("Instruction len issue"),
        }
    }
    println!("Res is {res}");
    Ok(())
}

fn question2() -> Result<()> {
    let lines = read_lines("inputs/input10.txt")?;
    // let lines = read_lines("inputs/test.txt")?;

    let mut instruction = vec!["".to_string()];
    let mut cycle = 0;
    let mut register: i32 = 1;
    let mut crt: Vec<char> = Vec::new();
    for line in lines {
        let line = line?;
        instruction = line.split_whitespace().map(|s| s.to_string()).collect();
        match instruction.len() {
            1 => {
                if (cycle % 40 == 0) {
                    cycle = 0;
                }
                draw(&mut crt, is_on_sprite(cycle, register));

                cycle += 1;
            }
            2 => {
                for _ in 0..2 {
                    if (cycle % 40 == 0) {
                        cycle = 0;
                    }
                    draw(&mut crt, is_on_sprite(cycle, register));

                    cycle += 1;
                }
                register += instruction[1].parse::<i32>().unwrap();
            }
            _ => println!("Instruction len issue"),
        }
    }
    print_crt(&crt);
    Ok(())
}

fn is_on_sprite(current_cycle: i32, current_register: i32) -> bool {
    let mut res = false;
    if current_cycle == current_register
        || current_cycle == current_register - 1
        || current_cycle == current_register + 1
    {
        res = true;
    }
    res
}

fn draw(vect: &mut Vec<char>, value: bool) {
    match value {
        true => vect.push('#'),
        false => vect.push('.'),
    }
}

fn print_crt(crt: &[char]) {
    println!();

    for (idx, val) in crt.iter().enumerate() {
        print!("{} ", val);
        if idx > 0 && (idx + 1) % 40 == 0 {
            // println!("INDEX IS {idx}");
            println!();
        }
    }
    println!();
}
