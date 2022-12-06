use eyre::Result;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<()> {
    // question1()?;
    question2()?;

    Ok(())
}

fn question1() -> Result<()> {
    let lines = read_lines("inputs/input6.txt")?;
    let mut _res = 0;
    for line in lines {
        let line = line?;
        for idx in 0..line.chars().count() + 1 {
            if idx > 3 && !all_different_in_substr(&line[idx - 4..idx]) {
                _res = idx;
                break;
            }
        }
    }
    println!("Result is {}", _res);
    Ok(())
}

fn question2() -> Result<()> {
    let lines = read_lines("inputs/input6.txt")?;
    let mut _res = 0;
    for line in lines {
        let line = line?;
        for idx in 0..line.chars().count() + 1 {
            if idx > 13 && all_different_in_substr(&line[idx - 14..idx]) {
                _res = idx;
                break;
            }
        }
    }
    println!("Result is {}", _res);
    Ok(())
}

fn all_different_in_substr(substr: &str) -> bool {
    let mut res = true;
    let mut tmp = substr.to_string();
    for (index, char) in substr.chars().enumerate() {
        tmp.remove(index).to_string();
        if tmp.contains(char) {
            res = false;
            break;
        }
        tmp = substr.to_string();
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
