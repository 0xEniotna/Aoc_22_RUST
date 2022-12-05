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
    let lines = read_lines("inputs/input5.txt")?;
    let mut sum = vec!["".to_string()];
    let mut counter = 0;

    let mut matrix = vec![vec![]; 9];

    for line in lines {
        let line = line?;

        if counter < 8 {
            let mut tmp = build_stacks(line.clone());
            let mut idx = 0;
            while idx < 9 {
                let pop = tmp.pop().unwrap();
                if pop != " " {
                    matrix[idx].push(pop);
                }
                idx += 1;
            }
        }
        if counter == 8 {
            matrix.reverse();
        }
        if counter > 9 {
            let tmp = line
                .replace("move", "")
                .replace("from", "")
                .replace("to", "");
            let res: Vec<u32> = tmp
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let from = res[1] as usize;
            let to = res[2] as usize;
            let amount = res[0];
            let mut ctr = 0;

            let mut tmp2 = matrix[from - 1].clone();
            let mut tmp3 = matrix[to - 1].clone();

            tmp2.reverse();
            tmp3.reverse();

            while ctr < amount {
                tmp3.push(tmp2.pop().unwrap());
                ctr += 1;
            }
            tmp2.reverse();
            tmp3.reverse();
            matrix[from - 1] = tmp2;
            matrix[to - 1] = tmp3;
        }
        counter += 1;
    }

    println!("End matrix  is: {:?}", matrix);
    Ok(())
}

fn question2() -> Result<()> {
    let lines = read_lines("inputs/input5.txt")?;
    let mut sum = vec!["".to_string()];
    let mut counter = 0;
    let mut matrix = vec![vec![]; 9];
    for line in lines {
        let line = line?;
        if counter < 8 {
            let mut tmp = build_stacks(line.clone());
            let mut idx = 0;
            while idx < 9 {
                let pop = tmp.pop().unwrap();
                if pop != " " {
                    matrix[idx].push(pop);
                }
                idx += 1;
            }
        }
        if counter == 8 {
            matrix.reverse();
        }
        if counter > 9 {
            let tmp = line
                .replace("move", "")
                .replace("from", "")
                .replace("to", "");
            let res: Vec<u32> = tmp
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let from = res[1] as usize;
            let to = res[2] as usize;
            let amount = res[0];
            let mut ctr = 0;
            // copy matrix from
            let mut tmp2 = matrix[from - 1].clone();
            // copy matrix to
            let mut tmp3 = matrix[to - 1].clone();

            tmp2.reverse();
            tmp3.reverse();
            // Collect the crates to transfer in tmp4.
            let mut tmp4: Vec<String> = Vec::new();
            while ctr < amount {
                tmp4.push(tmp2.pop().unwrap());
                ctr += 1;
            }
            // tmp4 is in the wrong order so reverse
            tmp4.reverse();
            tmp3.extend(tmp4);
            tmp2.reverse();
            tmp3.reverse();
            matrix[from - 1] = tmp2;
            matrix[to - 1] = tmp3;
        }
        counter += 1;
    }

    println!("End matrix  is: {:?}", matrix);
    Ok(())
}

fn build_stacks(line: String) -> Vec<String> {
    let mut matrix: Vec<String> = Vec::new();
    let mut counter = 1;
    while counter < line.chars().count() {
        matrix.push(line.chars().nth(counter).unwrap().to_string());
        counter += 4;
    }
    return matrix;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
