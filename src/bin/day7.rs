use eyre::Result;

use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<()> {
    question1()?;
    // question2()?;

    Ok(())
}

fn question1() -> Result<()> {
    let lines = read_lines("inputs/test.txt")?;
    let mut vec = Vec::new();
    let mut counter_ls_start = 0;
    let mut counter_ls_end = 0;
    let mut depth = 0;
    let mut home: TreeNode = TreeNode {
        value: Entry {
            name: "/".to_string(),
            size: 0,
        },
        depth: depth,
        children: vec![],
    };

    let mut last_folder = "".to_string();

    for (index, line) in lines.enumerate() {
        let line = line?;
        vec.push(line.clone());
        let line_split: Vec<&str> = line.split_whitespace().collect();
        println!("{:?}", line_split);
        if line_split[1] == "cd" {
            if line_split[2] == ".." {
                depth -= 1;
                continue;
            } else {
                depth += 1;

                last_folder = line_split[2].to_string();
                continue;
            }
        } else if line_split[0] == "$" && line_split[1] == "ls" {
            counter_ls_start = index + 1;
            continue;
        } else if line_split[0] != "dir" && line_split[0] != "$" {
            home.add_child(TreeNode {
                value: Entry {
                    name: line_split[1].to_string(),
                    size: line_split[0].parse::<i32>().unwrap(),
                },
                depth: depth,
                children: vec![],
            })
        } else if line_split[0] == "dir" {
            home.add_child(TreeNode {
                value: Entry {
                    name: line_split[1].to_string(),
                    size: 0,
                },
                depth: depth,
                children: vec![],
            })
        }
    }
    Ok(())
}

fn question2() -> Result<()> {
    let lines = read_lines("inputs/input6.txt")?;
    let mut _res = 0;
    for line in lines {
        let line = line?;
    }

    println!("Result is {}", _res);
    Ok(())
}

// enum Entry {
//     File {
//         name: String,
//         size: i32,
//     },
//     Folder {
//         name: String,
//         size: i32,
//         entries: HashMap<String, Entry>,
//     },
// }

struct Entry {
    name: String,
    size: i32,
}
struct TreeNode {
    value: Entry,
    depth: i32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(entry: Entry, _depth: i32) -> TreeNode {
        TreeNode {
            value: entry,
            depth: _depth,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    fn compute_node_size(&mut self) {
        let mut cumulative_size: i32 = 0;
        for child in &self.children {
            cumulative_size += get_size(&child.value)
        }
        self.value.size = cumulative_size;
    }

    fn get_value(&self) -> &Entry {
        &self.value
    }

    fn get_depth(&self) -> &i32 {
        &self.depth
    }
}

// fn get_name(entry: &Entry) -> String {
//     match entry {
//         Entry::File { name, .. } => name.to_string(),
//         Entry::Folder { name, .. } => name.to_string(),
//     }
// }

fn get_size(entry: &Entry) -> i32 {
    entry.size
}

fn create_entry(name: &str, size: i32) -> Entry {
    Entry {
        name: name.to_string(),
        size,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
