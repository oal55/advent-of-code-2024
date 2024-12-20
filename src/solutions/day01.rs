use std::collections::HashMap;
use std::io::BufRead;

use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (i64, i64) { (part1(file_path), part2(file_path)) }

fn part1(file_path: &str) -> i64 {
    let reader = file_reader(file_path);
    let mut nums_p: Vec<i64> = Vec::new();
    let mut nums_q: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        match parse_line(&line) {
            Ok((p, q)) => {
                nums_p.push(p);
                nums_q.push(q);
            },
            Err(_) => { 
                eprintln!("Bad line: \"{}\"", line);
                std::process::exit(1);
            }
        }
    }

    nums_p.sort();
    nums_q.sort();
    nums_p.iter().zip(nums_q.iter()).map(|(p, q)| (p - q).abs()).sum()
}

fn part2(file_path: &str) -> i64 {
    let reader = file_reader(file_path);
    let mut nums_p: Vec<i64> = Vec::new();
    let mut freq: HashMap<i64, i64> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        match parse_line(&line) {
            Ok((p, q)) => {
                nums_p.push(p);
                *freq.entry(q).or_insert(0) += 1;
            },
            Err(_) => { 
                eprintln!("Bad line: \"{}\"", line);
                std::process::exit(1);
            }
        }
    }
    nums_p.iter().map(|p| p * freq.get(p).unwrap_or(&0)).sum()
}


fn parse_line(line: &String) -> Result<(i64, i64), String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Input must contain exactly two space-separated numbers".to_string());
    }
    let num1 = parts[0].parse::<i64>().map_err(|_| "Failed to parse the first number")?;
    let num2 = parts[1].parse::<i64>().map_err(|_| "Failed to parse the second number")?;
    Ok((num1, num2))
}
