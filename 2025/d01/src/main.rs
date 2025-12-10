
const START: i32 = 50;
const INPUT: &str = include_str!("../day01.txt");

fn parse_line(line: &str) -> Option<(char, i32)> {
    if line.len() < 2 {
        return None;
    }
    let direction = line.chars().next()?;
    let clicks = line[1..].parse().ok()?;
    Some((direction, clicks))
}

fn comb1() {
    let mut current: i32 = START;
    let mut zero_count: i32 = 0;
    
    for line in INPUT.lines() {
        if current == 0 {zero_count += 1;}

        let line = line.trim();
        
        if line.is_empty() {continue;}

        let (direction, clicks) = parse_line(line)
            .expect("Invalid format");

        if direction == 'R' {
            current += clicks;
        }else{
            current -= clicks;
        }
        current = (100 + current) %100;
    }
    println!("code: {}", zero_count);
}

fn comb2(){
    let mut current: i32 = START;
    let mut zero_count: i32 = 0;
    
    for line in INPUT.lines() {
        let line = line.trim();
        
        if line.is_empty() {continue;}

        let (direction, clicks) = parse_line(line)
            .expect("Invalid format");

        let delta = if direction == 'R' {1} else {-1};
        for _ in 0..clicks{
            current = (current + delta + 100)%100;
            if current == 0 {zero_count +=1;}
        }
    }
    println!("second code: {}", zero_count);
}

fn main() {
    comb1();
    comb2();
}
