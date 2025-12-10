
const INPUT: &str = include_str!("../day02.txt");

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    &s[..half] == &s[half..]
}

fn is_invalid_id_general(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Need at least 2 repetitions → max period is len/2
    for period in 1..=len / 2 {
        if len % period != 0 {
            continue;
        }

        let k = len / period;
        if k < 2 {
            continue;
        }

        // Check if s[0..period] repeated k times equals s
        let base = &s[..period];
        if (0..k).all(|i| &s[i * period..(i + 1) * period] == base) {
            return true;
        }
    }
    false
}

fn count_invalid_ids(start: u64, end: u64) -> Vec<u64> {
    let mut pals: Vec<u64> = Vec::new();

    for n in start..=end{
        if is_invalid_id(n){
            pals.push(n);
        }
    }
    pals
}

fn count_invalid_ids_general(start: u64, end: u64) -> Vec<u64> {
    let mut pals: Vec<u64> = Vec::new();

    for n in start..=end{
        if is_invalid_id_general(n){
            pals.push(n);
        }
    }
    pals
}

fn main() {
    let mut total_pals: Vec<u64> = Vec::new();
    for range_str in INPUT.split(','){
        let range_str = range_str.trim();
        if range_str.is_empty() {continue;}

        // parse start and end
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            panic!("Invalid range: {}", range_str);
        }

        let start: u64 = parts[0].parse().expect("Invalid start");
        let end: u64 = parts[1].parse().expect("Invalid end");
        total_pals.extend(
            count_invalid_ids_general(start, end)
        );
        // println!("{:?}", total_pals);
    }
    let res: u64 = total_pals.into_iter().sum();
    println!("Sum of invalid IDS: {}", res);
}
