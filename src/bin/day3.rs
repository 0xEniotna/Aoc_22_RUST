use eyre::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<()> {
    question1()?;
    question2()?;

    Ok(())
}

fn question1() -> Result<()> {
    let lines = read_lines("inputs/input3.txt")?;
    let mut sum = 0;
    for line in lines {
        let line = line?;
        let double = find_doublon_by_line(line);

        if double.is_ascii_lowercase() {
            sum += double as u32 - 96;
        }
        if double.is_ascii_uppercase() {
            sum += double as u32 - 64 + 26;
        }
    }
    println!("score is: {sum}, should be 7878");
    Ok(())
}

fn question2() -> Result<()> {
    let lines = read_lines("inputs/input3.txt")?;
    let mut sum = 0;
    let mut string = "".to_owned();
    let mut counter = 0;

    for line in lines {
        counter += 1;
        let line = line?;
        string = [string, line.clone()].join("\n");

        if counter == 3 {
            let common = find_common_by_group(string.to_string());

            if common.is_ascii_lowercase() {
                sum += common as u32 - 96;
            }
            if common.is_ascii_uppercase() {
                sum += common as u32 - 64 + 26;
            }
            counter = 0;
            string = "".to_owned();
        }
    }

    println!("score is: {sum}");
    Ok(())
}

fn find_doublon_by_line(line: String) -> char {
    let mut doublon = '0';
    let (first, last) = line.split_at(line.chars().count() / 2);

    for i in 0..(first.chars().count()) {
        if last.contains(first.chars().nth(i).unwrap()) {
            doublon = first.chars().nth(i).unwrap();
            break;
        }
    }
    doublon
}

fn find_common_by_group(line: String) -> char {
    let mut doublon = '0';
    let mut string = line.lines();

    let _tmp = string.next().unwrap();
    let first = string.next().unwrap();
    let middle = string.next().unwrap();
    let last = string.next().unwrap();

    for i in 0..(first.chars().count()) {
        if last.contains(first.chars().nth(i).unwrap())
            && middle.contains(first.chars().nth(i).unwrap())
        {
            doublon = first.chars().nth(i).unwrap();
            break;
        }
    }
    doublon
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
