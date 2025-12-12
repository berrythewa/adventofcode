const INPUT: &str = include_str!("../day03.txt");

fn max_joltage(bank: &str) -> i32 {
    let digits: Vec<u8> = bank
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    let mut max_val = 0;
    for i in 0..digits.len() {
        for j in i + 1..digits.len() {
            let val = 10 * digits[i] as i32 + digits[j] as i32;
            if val > max_val {
                max_val = val;
            }
        }
    }
    max_val
}


fn max_joltage_12(bank: &str) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let n = digits.len();
    let target_len = 12;

    // If the bank has fewer than 12 digits, the problem is ill-defined,
    // but AoC guarantees at least 12.
    if n < target_len {
        // Fallback: pad with zeros or panic? But not expected.
        panic!("Bank too short: {}", bank);
    }

    let mut to_remove = n - target_len;
    let mut stack: Vec<char> = Vec::with_capacity(n);

    for &digit in &digits {
        // While we can remove, and stack isn't empty, and current digit is larger than top
        while to_remove > 0 && !stack.is_empty() && digit > *stack.last().unwrap() {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(digit);
    }

    // If we still need to remove (e.g., non-decreasing sequence), truncate from end
    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    // Take exactly the first 12 digits
    let result_str: String = stack.iter().take(target_len).collect();

    // Parse to u64
    result_str.parse().unwrap()
}

fn main() {
    let mut sum_joltage: u64 = 0;

    for bank in INPUT.lines() {
        sum_joltage += max_joltage_12(bank);

    }
    println!("MAX SUM JOLT: {}", sum_joltage);
}
