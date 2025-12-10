use std::env;
use std::fs;
use std::process;
use std::io::{self,BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    let file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error readingfile '{}' : {}", filename, error);
            process::exit(1);
        }

    };

    let reader = BufReader::new(file);

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for (n, line) in reader.lines().enumerate(){
        match line {
            Ok(line) => {
                let numbers: Vec<&str> = line.split_whitespace().collect();
                if numbers.len() < 2{
                    eprintln!("line {} wrong format", n+1);
                    continue;
                }

                match(numbers[0].parse::<i32>(), numbers[1].parse::<i32>()){
                    (Ok(num1), Ok(num2)) => {
                        left.push(num1);
                        right.push(num2);
                    }
                    _ => {
                        eprintln!("line {}: Parsing error from {}", n+1, line);
                    }
                }

            }
            Err(error) => {
                eprintln!("Error reading line number {}: {}", n, error);
                process::exit(1);
            }
        }
    }
    left.sort();
    right.sort();
    
    let distance_sum: i32 =  left.iter().zip(right.iter())
        .map(|(n1,n2)| (n2 - n1).abs())
        .sum();

    println!("Total distance: {}", distance_sum);
    Ok(())
}
