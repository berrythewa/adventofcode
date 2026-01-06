// part 1
// const  INPUT: &str = include_str!("../day05.txt");


// fn check_fresh_ids(target: u64) -> bool{    
//     for line in INPUT.lines(){
//         if line.is_empty(){break;}
        
//         let parts: Vec<&str> = line.split('-').collect();
//         let start = parts[0].parse::<u64>().unwrap();
//         let end  = parts[1].parse::<u64>().unwrap();
//         if target >= start && target <= end {return true;}
//     }
//     return false;
// }




// fn main() {

//     let mut count = 0;


//     for line in INPUT.lines(){
        
//         if line.contains("-") || line.is_empty(){
//             continue;
//         }else{
//             let curr = line.trim().parse::<u64>().unwrap();
//             if check_fresh_ids(curr) {
//                 count += 1
//             }
//        }

//     }
//     // println!("fresh id count: {}, {:?}", available.len(), available);
//     println!("fresh id count: {}", count);

// }

// part 2
fn count_unique_ids_from_ranges(ranges: &[(i64, i64)]) -> i64 {
    if ranges.is_empty() {
        return 0;
    }

    let mut sorted = ranges
        .to_vec().
        .sort_unstable_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current_start = sorted[0].0;
    let mut current_end = sorted[0].1;

    for &(start, end) in sorted.iter().skip(1) {
        if start <= current_end + 1 {
            // Overlap or adjacent → merge
            current_end = current_end.max(end);
        } else {
            // No overlap → push current, start new
            merged.push((current_start, current_end));
            current_start = start;
            current_end = end;
        }
    }
    merged.push((current_start, current_end));

    // 3. Sum lengths (inclusive: end - start + 1)
    merged.iter().map(|&(s, e)| e - s + 1).sum()
}

fn main() {
    let input = include_str!("../day05.txt");
    let ranges: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| line.contains("-"))
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse().expect("Invalid start");
            let end = parts[1].parse().expect("Invalid end");
            (start, end)
        })
        .collect();

    let total = count_unique_ids_from_ranges(&ranges);
    println!("{}", total);
}