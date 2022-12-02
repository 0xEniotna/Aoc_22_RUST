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
    let mut cumulative_score = 0;
    let score_matrix_shapes = [1, 2, 3];
    let score_matrix_result = [0, 3, 6];
    let lines = read_lines("inputs/input2.txt")?;
    for line in lines {
        let line = line?;
        match line.chars().nth(0).unwrap() {
            'A' => match line.chars().nth(2).unwrap() {
                'X' => cumulative_score += score_matrix_shapes[0] + score_matrix_result[1],
                'Y' => cumulative_score += score_matrix_shapes[1] + score_matrix_result[2],
                'Z' => cumulative_score += score_matrix_shapes[2] + score_matrix_result[0],
                _ => println!("{}", "Error"),
            },
            'B' => match line.chars().nth(2).unwrap() {
                'X' => cumulative_score += score_matrix_shapes[0] + score_matrix_result[0],
                'Y' => cumulative_score += score_matrix_shapes[1] + score_matrix_result[1],
                'Z' => cumulative_score += score_matrix_shapes[2] + score_matrix_result[2],
                _ => println!("{}", "Error"),
            },
            'C' => match line.chars().nth(2).unwrap() {
                'X' => cumulative_score += score_matrix_shapes[0] + score_matrix_result[2],
                'Y' => cumulative_score += score_matrix_shapes[1] + score_matrix_result[0],
                'Z' => cumulative_score += score_matrix_shapes[2] + score_matrix_result[1],
                _ => println!("{}", "Error"),
            },
            _ => println!("{}", "Error"),
        }
    }
    println!("score is: {cumulative_score}");
    Ok(())
}

fn question2() -> Result<()> {
    let mut cumulative_score = 0;
    let score_matrix_shapes = [1, 2, 3];
    let score_matrix_result = [0, 3, 6];
    let lines = read_lines("inputs/input2.txt")?;
    for line in lines {
        let line = line?;
        match line.chars().nth(0).unwrap() {
            'A' => match line.chars().nth(2).unwrap() {
                'X' => cumulative_score += score_matrix_shapes[2] + score_matrix_result[0],
                'Y' => cumulative_score += score_matrix_shapes[0] + score_matrix_result[1],
                'Z' => cumulative_score += score_matrix_shapes[1] + score_matrix_result[2],
                _ => println!("{}", "Error"),
            },
            'B' => match line.chars().nth(2).unwrap() {
                'X' => cumulative_score += score_matrix_shapes[0] + score_matrix_result[0],
                'Y' => cumulative_score += score_matrix_shapes[1] + score_matrix_result[1],
                'Z' => cumulative_score += score_matrix_shapes[2] + score_matrix_result[2],
                _ => println!("{}", "Error"),
            },
            'C' => match line.chars().nth(2).unwrap() {
                'X' => cumulative_score += score_matrix_shapes[1] + score_matrix_result[0],
                'Y' => cumulative_score += score_matrix_shapes[2] + score_matrix_result[1],
                'Z' => cumulative_score += score_matrix_shapes[0] + score_matrix_result[2],
                _ => println!("{}", "Error"),
            },
            _ => println!("{}", "Error"),
        }
    }
    println!("2 - score is: {cumulative_score}");
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
