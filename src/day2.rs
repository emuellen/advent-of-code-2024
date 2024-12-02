use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log::{debug, info};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Init,
    Increase,
    Decrease,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum RejectionReason {
    None,
    DirectionChange,
    SameScore,
    OutOfRange,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./input/day2.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut safe_reports = 0;
    let mut safe_reports_error_dampener = 0;
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        let mut scores = vec![];

        while let Some(score) = parts.next() {
            let score: i32 = score.parse()?;
            scores.push(score);
        }

        let rejection_reason = test_scores(&scores);

        if rejection_reason.eq(&RejectionReason::None) {
            safe_reports = safe_reports + 1;
            continue;
        }

        if !rejection_reason.eq(&RejectionReason::None) {
            // Problem dampener
            for i in 0..scores.len() {
                let mut new_scores = scores.clone();
                new_scores.remove(i);
                let rejection_reason = test_scores(&new_scores);
                if rejection_reason.eq(&RejectionReason::None) {
                    safe_reports_error_dampener = safe_reports_error_dampener + 1;
                    break;
                }
            }
            debug!("{line} : {rejection_reason:?}");
        }
    }
    info!("Number of safe reports: {safe_reports}");
    info!(
        "Number of safe reports with error dampener: {}",
        safe_reports + safe_reports_error_dampener
    );
    Ok(())
}

fn test_scores(scores: &[i32]) -> RejectionReason {
    let mut previous_score = 0;
    let mut direction = Direction::Init;
    let mut rejection_reason = RejectionReason::None;
    let mut direction_set = false;
    for score in scores {
        let current_direction = match score.cmp(&previous_score) {
            std::cmp::Ordering::Less => Direction::Decrease,
            std::cmp::Ordering::Greater => Direction::Increase,
            std::cmp::Ordering::Equal => {
                rejection_reason = RejectionReason::SameScore;
                break;
            }
        };

        if *score != 0 && previous_score != 0 && !direction_set {
            direction = current_direction;
            direction_set = true;
        }

        if previous_score != 0 && !(1..=3).contains(&(score - previous_score).abs()) {
            rejection_reason = RejectionReason::OutOfRange;
            break;
        }

        previous_score = *score;

        if direction.eq(&Direction::Init) {
            continue;
        }

        if !direction.eq(&current_direction) {
            rejection_reason = RejectionReason::DirectionChange;
            break;
        }
    }
    rejection_reason
}

#[test]
fn test_run() {
    run().unwrap();
}
