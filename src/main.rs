use eyre::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<()> {
    // question_1()?;
    question_2()?;
    Ok(())
}

fn question_1() -> Result<()> {
    let lines = read_lines("src/input.txt")?;
    let mut most_calories = 0;
    let mut last_processed_elve_cumulative_calories = 0;

    for line in lines {
        let line = line?;
        if (line.is_empty()) {
            if last_processed_elve_cumulative_calories > most_calories {
                most_calories = last_processed_elve_cumulative_calories;
            }
            last_processed_elve_cumulative_calories = 0;
        } else {
            last_processed_elve_cumulative_calories += line.parse::<u32>()?;
        }
    }
    println!("Most calories: {}", most_calories);
    Ok(())
}

fn question_2() -> Result<()> {
    let lines = read_lines("src/input.txt")?;
    let mut top_3_elf = [0, 0, 0];
    let mut last_processed_elve_cumulative_calories = 0;
    let mut most_calories = 0;

    for line in lines {
        let line = line?;
        if (line.is_empty()) {
            for (index, calories) in top_3_elf.iter().enumerate() {
                if last_processed_elve_cumulative_calories > *calories {
                    top_3_elf[index] = last_processed_elve_cumulative_calories;
                    break;
                }
            }
            last_processed_elve_cumulative_calories = 0;
        } else {
            last_processed_elve_cumulative_calories += line.parse::<u32>()?;
        }
    }
    for calo in top_3_elf {
        most_calories += calo;
        println!("Top 3: {}", calo);
    }
    println!("Most calories: {}", most_calories);
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
