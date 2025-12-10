use std::env;
use std::fs;
use std::process;
use std::io::{BufRead, BufReader};


fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let first_diff = numbers[0] - numbers[1];
    if first_diff.abs() > 3 || first_diff == 0{ return false; }
    let sign = first_diff.signum(); //1 for incr -1 for decr
    for pair in numbers.windows(2) {
        let diff = pair[0] - pair[1];

        if diff.abs() > 3 || diff == 0 {return false;}
        if diff.signum() != sign {return false;}
    }
    return true;
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut safe_counter = 0;

    for (i, line) in reader.lines().enumerate(){
        let line  = line?;
        let numbers: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect();

        match numbers {
            Ok(nums) if nums.len() >= 2 => {
                if is_safe(&nums){
                    safe_counter += 1;
                }
            
            },
            Ok(_) => {
                eprintln!("line {}: Too few numbers", i + 1);
                process::exit(1);
            },
            Err(e) => {
                eprintln!("line {}: parse error: {}", i + 1, e);
                process::exit(1);
            }
        }
    }
    println!("safe: {} ", safe_counter);
    Ok(())
}
