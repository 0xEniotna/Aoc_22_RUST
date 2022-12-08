use eyre::Result;

use std::array;
use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::{self, BufRead};
use std::iter::Enumerate;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_visible_left(array: &[Vec<i16>], idx_row: usize, idx_col: usize) -> bool {
    let mut res = true;
    for i in 0..idx_col {
        if array[idx_row][i] >= array[idx_row][idx_col] {
            res = false;
            break;
        }
    }

    res
}
fn is_visible_right(array: &[Vec<i16>], idx_row: usize, idx_col: usize) -> bool {
    let mut res = true;
    for i in idx_col + 1..array[0].len() {
        if array[idx_row][i] >= array[idx_row][idx_col] {
            res = false;
            break;
        }
    }

    res
}

fn is_visible_top(array: &[Vec<i16>], idx_row: usize, idx_col: usize) -> bool {
    let mut res = true;
    if idx_row != 0 {
        for i in 0..idx_row {
            if (array[i][idx_col] >= array[idx_row][idx_col]) {
                res = false;
                break;
            }
        }
    }
    res
}

fn is_visible_bottom(array: &[Vec<i16>], idx_row: usize, idx_col: usize) -> bool {
    let mut res = true;
    if idx_row != array.len() - 1 {
        for i in idx_row + 1..array.len() {
            if array[i][idx_col] >= array[idx_row][idx_col] {
                res = false;
                break;
            }
        }
    }

    res
}

fn count_left(array: &[Vec<i16>], idx_row: usize, idx_col: usize) -> i32 {
    let mut res = 0;
    if idx_col != 0 {
        let mut counter = idx_col - 1;
        while array[idx_row][counter] < array[idx_row][idx_col] {
            res += 1;
            if counter == 0 {
                break;
            }
            counter -= 1;
        }
        // IF IT'S TALLER
        if array[idx_row][counter] >= array[idx_row][idx_col] {
            res += 1;
        }
    }
    res
}
fn count_right(array: &[Vec<i16>], idx_row: usize, idx_col: usize) -> i32 {
    let mut res = 0;
    for i in idx_col + 1..array[0].len() {
        if array[idx_row][i] >= array[idx_row][idx_col] {
            res += 1;
            break;
        }
        res += 1;
    }

    res
}

fn count_top(array: &[Vec<i16>], idx_row: usize, idx_col: usize) -> i32 {
    let mut res = 0;
    if idx_row != 0 {
        let mut counter = idx_row - 1;
        while array[counter][idx_col] < array[idx_row][idx_col] {
            res += 1;
            if counter == 0 {
                break;
            }
            counter -= 1;
        }
        // IF IT'S TALLER
        if array[counter][idx_col] >= array[idx_row][idx_col] {
            res += 1;
        }
    }
    res
}

fn count_bottom(array: &[Vec<i16>], idx_row: usize, idx_col: usize) -> i32 {
    let mut res = 0;

    if idx_row != array.len() - 1 {
        for i in idx_row + 1..array.len() {
            if array[i][idx_col] >= array[idx_row][idx_col] {
                res += 1;
                break;
            }
            res += 1;
        }
    }

    res
}

fn main() -> Result<()> {
    // question1()?;
    question2()?;

    Ok(())
}

fn question1() -> Result<()> {
    let lines = read_lines("inputs/input8.txt")?;
    // let lines = read_lines("inputs/test.txt")?;

    let mut array_vec: Vec<Vec<i16>> = Vec::new();
    let mut res = 0;
    for line in lines {
        let line = line?;
        let mut array_rows: Vec<i16> = Vec::new();

        for char in line.chars() {
            array_rows.push(char.to_string().parse().unwrap());
        }
        array_vec.push(array_rows);
    }
    for i in 0..array_vec.len() {
        for j in 0..array_vec[0].len() {
            if is_visible_bottom(&array_vec, i, j)
                || is_visible_top(&array_vec, i, j)
                || is_visible_left(&array_vec, i, j)
                || is_visible_right(&array_vec, i, j)
            {
                res += 1;
            }
        }
    }
    println!("Res is {res}");
    Ok(())
}

fn question2() -> Result<()> {
    let lines = read_lines("inputs/input8.txt")?;
    // let lines = read_lines("inputs/test.txt")?;

    let mut array_vec: Vec<Vec<i16>> = Vec::new();
    let mut res: i32 = 0;
    for line in lines {
        let line = line?;
        let mut array_rows: Vec<i16> = Vec::new();

        for char in line.chars() {
            array_rows.push(char.to_string().parse().unwrap());
        }
        array_vec.push(array_rows);
    }
    for i in 0..array_vec.len() {
        for j in 0..array_vec[0].len() {
            let bot = count_bottom(&array_vec, i, j);
            let top = count_top(&array_vec, i, j);
            let left = count_left(&array_vec, i, j);
            let right = count_right(&array_vec, i, j);
            let product = bot * top * left * right;
            if res < product {
                res = product;
            }
        }
    }
    println!("Res is {res}");
    Ok(())
}
