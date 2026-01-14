const INPUT: &str = include_str!("../day06.txt");

//part 1

// fn main() {
//     let mut grand_total: Vec<u64> = Vec::new();

//     let mut grid: Vec<Vec<&str>> = Vec::new();

//     for line in INPUT.lines() {
        
//         let numbers: Vec<&str> = line.split_whitespace().collect();
//         grid.push(numbers);   
//         // println!("{:?}", numbers);
//     }

//     let num_rows = grid.len();
//     let operator_row = num_rows - 1;

//     let mut col = 0;
//     while col < grid[0].len() {
       
//         let mut row = 1;
//         let mut total = grid[row-1][col].parse::<u64>().unwrap();
//         while row < operator_row {
//             // println!("total {} ", total);
//             if grid[operator_row][col] == "+"{
//                 total += grid[row][col].parse::<u64>().unwrap();
//             }else if grid[operator_row][col] == "*"{
//                 total *= grid[row][col].parse::<u64>().unwrap();
//             }
//             row += 1;
//         }
//         // println!("current total {} col {}", total, col);
//         grand_total.push(total);
//         col += 1;
//     }

//     // println!("{:?}", grid);
//     println!("Grand Total: {}", grand_total.iter().sum::<u64>());
// }


// part 2

fn main(){
    let grid : Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut numbers: Vec<u64> = Vec::new();
    let mut col: u32 = (grid[0].len() - 1).try_into().unwrap();
    let operator_row = grid.last().clone().unwrap();
    let mut memo : Vec<u64> = Vec::new();
    while col >= 0 {
        let mut row = 0;
        let mut number = String::new();
        while row < grid.len() -1 {
            if !grid[row][col as usize].is_whitespace(){
                number.push(grid[row][col as usize]);
            }
            row += 1;
        }
        if !number.is_empty(){
            numbers.push(number.parse::<u64>().unwrap());
        }
        if col < operator_row.len() as u32 && operator_row[col as usize] != ' ' {
            if operator_row[col as usize] == '+'{
                memo.push(numbers.iter().sum());
            }else if operator_row[col as usize] == '*'{
                memo.push(numbers.iter().product());
            }
            numbers.clear();
        }
        if col > 0{
            col -= 1;
        }else{
            break;
        }
       
    }
    
    println!("total sum : {}", memo.iter().sum::<u64>());
}