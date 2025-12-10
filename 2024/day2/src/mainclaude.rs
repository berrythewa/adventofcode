use std::env;
use std::fs;
use std::process;
use std::io::{BufRead, BufReader};

fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }
    
    let mut increasing = None;
    for pair in numbers.windows(2) {
        let diff = pair[1] - pair[0];
        
        // Check if difference is within bounds
        if diff.abs() > 3 || diff == 0 {
            return false;
        }
        
        // Determine and check direction
        match increasing {
            None => increasing = Some(diff > 0),
            Some(inc) => if (diff > 0) != inc {
                return false;
            }
        }
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut safe_counter = 0;
    
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let numbers: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect();
            
        match numbers {
            Ok(nums) if nums.len() >= 2 => {
                if is_safe(&nums) {
                    safe_counter += 1;
                }
            },
            Ok(_) => {
                eprintln!("Line {}: Too few numbers", i + 1);
                process::exit(1);
            },
            Err(e) => {
                eprintln!("Line {}: Parse error: {}", i + 1, e);
                process::exit(1);
            }
        }
    }
    
    println!("Safe reports: {}", safe_counter);
    Ok(())
}
