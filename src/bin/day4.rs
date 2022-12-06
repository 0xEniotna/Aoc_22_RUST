use eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<()> {
    question1()?;
    question2()?;

    Ok(())
}

fn question1() -> Result<()> {
    let lines = read_lines("inputs/input4.txt")?;
    let mut sum = 0;
    for line in lines {
        let line = line?;
        if is_contained_in(line) {
            sum += 1;
        }
    }
    println!("score is: {sum}");
    Ok(())
}

fn question2() -> Result<()> {
    let lines = read_lines("inputs/input4.txt")?;
    let mut sum = 0;
    for line in lines {
        let line = line?;
        if is_overlapping(line) {
            sum += 1;
        }
    }
    println!("score is: {sum}");
    Ok(())
}

fn is_contained_in(line: String) -> bool {
    let vec: Vec<&str> = line.split(',').collect();
    let first: Vec<&str> = vec[0].split('-').collect();
    let second: Vec<&str> = vec[1].split('-').collect();
    let first_a = first[0].parse::<i32>().unwrap();
    let first_b = first[1].parse::<i32>().unwrap();
    let second_a = second[0].parse::<i32>().unwrap();
    let second_b = second[1].parse::<i32>().unwrap();
    
    (first_a >= second_a && first_b <= second_b)
        || (first_a <= second_a && first_b >= second_b)
}

fn is_overlapping(line: String) -> bool {
    let vec: Vec<&str> = line.split(',').collect();
    let first: Vec<&str> = vec[0].split('-').collect();
    let second: Vec<&str> = vec[1].split('-').collect();
    let first_a = first[0].parse::<i32>().unwrap();
    let first_b = first[1].parse::<i32>().unwrap();
    let second_a = second[0].parse::<i32>().unwrap();
    let second_b = second[1].parse::<i32>().unwrap();
    // +1 because 0..n goes to n-1, we want to include n
    let vec1 = (first_a..first_b + 1_i32).collect::<Vec<_>>();
    let vec2 = (second_a..second_b + 1_i32).collect::<Vec<_>>();

    let set1: HashSet<&i32> = vec1.iter().collect();
    let set2: HashSet<&i32> = vec2.iter().collect();

    !set1.is_disjoint(&set2)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
