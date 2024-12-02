use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log::info;

pub fn run() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./input/day1.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let Some(a) = parts.next() {
            vec_a.push(a.parse::<i32>().unwrap());
        }
        if let Some(b) = parts.next() {
            vec_b.push(b.parse::<i32>().unwrap());
        }
    }

    vec_a.sort();
    vec_b.sort();

    let differences: i32 = vec_a.iter().zip(&vec_b).map(|(a, b)| (b - a).abs()).sum();
    info!("Differences: {:?}", differences);

    let similarity_score: i32 = vec_a
        .iter()
        .map(|a| a * vec_b.iter().filter(|&&b| b == *a).count() as i32)
        .sum();

    info!("Similarity Score: {:?}", similarity_score);

    Ok(())
}
