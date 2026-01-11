const INPUT: &str = include_str!("../day06.txt");


fn main() {
    let mut grand_total: Vec<u64> = Vec::new();

    let mut grid: Vec<Vec<&str>> = Vec::new();

    for line in INPUT.lines() {
        
        let numbers: Vec<&str> = line.split_whitespace().collect();
        grid.push(numbers);   
        // println!("{:?}", numbers);
    }

    let num_rows = grid.len();
    let operator_row = num_rows - 1;

    let mut col = 0;
    while col < grid[0].len() {
       
        let mut row = 1;
        let mut total = grid[row-1][col].parse::<u64>().unwrap();
        while row < operator_row {
            // println!("total {} ", total);
            if grid[operator_row][col] == "+"{
                total += grid[row][col].parse::<u64>().unwrap();
            }else if grid[operator_row][col] == "*"{
                total *= grid[row][col].parse::<u64>().unwrap();
            }
            row += 1;
        }
        // println!("current total {} col {}", total, col);
        grand_total.push(total);
        col += 1;
    }

    // println!("{:?}", grid);
    println!("Grand Total: {}", grand_total.iter().sum::<u64>());
}
