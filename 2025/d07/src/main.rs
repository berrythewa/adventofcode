// use chars_counter::{ICharsCounter, ICharCounterExt};

const INPUT: &str = include_str!("../day07.txt");

// part 1
// fn main() {
//     let mut n_splits: u64 = 0;
    
//     let mut grid : Vec<Vec<char>> = INPUT
//         .lines()
//         .map(|line| line.chars().collect())
//         .collect();

//     for i in 0..grid.len() {
//         for j in 0..grid[i].len(){
//             if grid[i][j] == 'S'{
//                 // source
//                 if i+1 < grid.len(){
//                     grid[i+1][j] = '|';
//                 }
//             }
//             if grid[i][j] == '|'{
//                 if i+1< grid.len() {
//                     if grid[i+1][j] == '^'{
//                         // split
//                         if j-1 > 0{ grid[i+1][j-1]='|'; }
//                         if j+1 < grid[i+1].len() { grid[i+1][j+1]='|'; }
//                         n_splits += 1;
//                     }
//                     else{
//                         // continue
//                         grid[i+1][j] = '|';
//                     }
//                 }
                
                
//             }
//         }
//     }

    
//     println!("number of splits {}", n_splits);
// }


fn main() {
    let mut n_timelines: u64 = 0;
    
    let mut grid : Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for i in 0..grid.len() {
        for j in 0..grid[i].len(){
            if grid[i][j] == 'S'{
                // source
                if i+1 < grid.len(){
                    grid[i+1][j] = '|';
                }
            }
            if grid[i][j] == '|'{
                if i+1< grid.len() {
                    if grid[i+1][j] == '^'{
                        // split
                        if j-1 > 0{ grid[i+1][j-1]='|'; n_timelines += 1;}
                        if j+1 < grid[i+1].len() { grid[i+1][j+1]='|'; n_timelines += 1;}
                    }
                    else{
                        // continue
                        grid[i+1][j] = '|';
                    }
                }
                
                
            }
        }
    }

    
    println!("number of timelines {}", n_timelines);
}
