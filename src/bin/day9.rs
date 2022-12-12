use eyre::Result;

use std::collections::HashMap;
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

struct Instruction {
    side: String,
    counter: i16,
}

struct Point {
    x: i16,
    y: i16,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn check_adjacent(H: &Point, T: &Point) -> bool {
    (H.x - T.x).abs() <= 1 && (H.y - T.y).abs() <= 1
}

fn main() -> Result<()> {
    // question1()?;
    question2()?;

    Ok(())
}

fn question1() -> Result<()> {
    let lines = read_lines("inputs/input9.txt")?;
    // let lines = read_lines("inputs/test.txt")?;
    let mut visited: Vec<Point> = Vec::new();

    let mut res = 0;
    let mut h: Point = Point { x: 0, y: 0 };
    let mut t: Point = Point { x: 0, y: 0 };
    visited.push(Point { x: t.x, y: t.y });
    for line in lines {
        let line = line?;
        let mut line_split = line.split_whitespace();
        let direction = Instruction {
            side: line_split.next().unwrap().to_string(),
            counter: line_split.next().unwrap().parse().unwrap(),
        };
        let mut steps = 1;
        while steps <= direction.counter {
            match direction.side.as_str() {
                "R" => {
                    h.x += 1;
                    if !check_adjacent(&h, &t) {
                        t.y = h.y;
                        t.x = h.x - 1;
                    }
                }
                "L" => {
                    h.x -= 1;
                    if !check_adjacent(&h, &t) {
                        t.y = h.y;
                        t.x = h.x + 1;
                    }
                }
                "U" => {
                    h.y += 1;
                    if !check_adjacent(&h, &t) {
                        t.y = h.y - 1;
                        t.x = h.x;
                    }
                }
                "D" => {
                    h.y -= 1;
                    if !check_adjacent(&h, &t) {
                        t.y = h.y + 1;
                        t.x = h.x;
                    }
                }
                _ => println!("Match error"),
            }
            if !visited.contains(&t) {
                visited.push(Point { x: t.x, y: t.y });
            }
            steps += 1;
        }
    }
    res = visited.len();
    println!("Res is {res}");
    Ok(())
}

fn question2() -> Result<()> {
    let lines = read_lines("inputs/input9.txt")?;
    // let lines = read_lines("inputs/test.txt")?;
    let mut visited: Vec<Point> = Vec::new();
    let mut snake: HashMap<i16, Point> = HashMap::new();
    for i in 0..11 {
        snake.insert(i, Point { x: 0, y: 0 });
    }
    let mut res = 0;
    let mut h: Point = Point { x: 0, y: 0 };
    let mut t: Point = Point { x: 0, y: 0 };
    visited.push(Point {
        x: snake.get(&9).unwrap().x,
        y: snake.get(&9).unwrap().y,
    });
    for line in lines {
        let line = line?;
        let mut line_split = line.split_whitespace();
        let direction = Instruction {
            side: line_split.next().unwrap().to_string(),
            counter: line_split.next().unwrap().parse().unwrap(),
        };
        let mut steps = 1;
        while steps <= direction.counter {
            match direction.side.as_str() {
                "R" => {
                    snake.get_mut(&0).unwrap().x += 1;
                    for i in 1..snake.len() + 1 {
                        if !check_adjacent(
                            snake.get(&(i as i16)).unwrap(),
                            snake.get(&((i - 1) as i16)).unwrap(),
                        ) {
                            t.y = h.y;
                            t.x = h.x + 1;
                        }
                    }
                }
                "L" => {
                    snake.get_mut(&0).unwrap().x -= 1;
                    for i in 1..snake.len() + 1 {
                        if !check_adjacent(
                            snake.get(&(i as i16)).unwrap(),
                            snake.get(&((i - 1) as i16)).unwrap(),
                        ) {
                            t.y = h.y;
                            t.x = h.x - 1;
                        }
                    }
                }
                "U" => {
                    snake.get_mut(&0).unwrap().y += 1;
                    for i in 1..snake.len() + 1 {
                        if !check_adjacent(
                            snake.get(&(i as i16)).unwrap(),
                            snake.get(&((i - 1) as i16)).unwrap(),
                        ) {
                            t.y = h.y - 1;
                            t.x = h.x;
                        }
                    }
                }
                "D" => {
                    snake.get_mut(&0).unwrap().y -= 1;
                    for i in 1..snake.len() + 1 {
                        if !check_adjacent(
                            snake.get(&(i as i16)).unwrap(),
                            snake.get(&((i - 1) as i16)).unwrap(),
                        ) {
                            t.y = h.y;
                            t.x = h.x - 1;
                        }
                    }
                }
                _ => println!("Match error"),
            }
            if !visited.contains(&t) {
                visited.push(Point { x: t.x, y: t.y });
            }
            steps += 1;
        }
    }
    res = visited.len();
    println!("Res is {res}");
    Ok(())
}
