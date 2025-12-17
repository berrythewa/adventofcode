
const INPUT: &str = include_str!("../day04.txt");

const DIRS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),           ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn parse_grid() -> Vec<Vec<char>>{
    let grid: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    grid
}

fn remove_accessibles(grid: &mut Vec<Vec<char>>) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = grid.clone();
    let mut valid_rolls: i32 = 0;


    for i in 0..rows{
        for j in 0..cols {
            if new_grid[i as usize][j as usize] != '@' {continue;}
            let adj_count = DIRS.iter()
                .map(|&(dr, dc)|{
                    let ni = i as i32 + dr;
                    let nj = j as i32 + dc;
                    if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32{
                        (new_grid[ni as usize][nj as usize] == '@') as u32
                    }else{
                        0
                    }
                }).sum::<u32>();
            if adj_count < 4 {
                valid_rolls += 1;
                new_grid[i as usize][j as usize] = '.';
            }
        }
    }
    *grid = new_grid;
    return valid_rolls as u32;
    
}

fn main() {
    let mut grid = parse_grid();
    let mut removed: u32 = 0;

    loop {
        let selected = remove_accessibles(&mut grid);
        removed += selected;
        if selected == 0{break;}
    }
    println!("removed rools: {}", removed);
}
