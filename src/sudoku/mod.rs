fn print_sudoku(sudoku: [[u8; 9]; 9]) {
    for i in 0..9 {
        if i % 3 == 0 && i != 0 {
            println!("------+-------+------");

        }
        for j in 0..9 {
            if j % 3 == 0 && j != 0 {
                print!("| ");
            }
            print!("{} ", sudoku[i][j]);
        }
        println!();
    }
}

fn is_valid(sudoku: [[u8; 9]; 9], num: u8, row: u8, col: u8) -> bool {
    for i in 0..9 {
        if sudoku[row as usize][i] == num || sudoku[i][col as usize] == num {
            false;
        }
    }
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku[(start_row + i) as usize][(start_col + j) as usize] == num {
                return false;
            }
        }
    }
    true
}

fn solve(sudoku: &mut [[u8; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if sudoku[row][col] == 0 {
                for num in 1..10 {
                    if is_valid(*sudoku, num, row as u8, col as u8) {
                        sudoku[row][col] = num;
                        if solve(sudoku) {
                            return true;
                        }
                    }
                    sudoku[row][col] = 0;
                }
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut sudoku: [[u8; 9]; 9] = [
        [0, 9, 0, 8, 0, 7, 0, 4, 0],
        [0, 1, 0, 4, 0, 0, 0, 9, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 8],
        [0, 2, 0, 0, 5, 0, 0, 0, 0],
        [0, 0, 9, 0, 3, 2, 0, 0, 0],
        [0, 5, 0, 0, 0, 0, 0, 0, 3],
        [0, 0, 0, 0, 0, 0, 0, 7, 1],
        [0, 0, 6, 0, 7, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0]
    ];

    println!("=====================");
    println!("Unsolved Sudoku: ");

    print_sudoku(sudoku);

    solve(&mut sudoku);
    println!("=====================");
    println!("=====================");
    println!("Solved Sudoku: ");
    print_sudoku(sudoku);
    println!("=====================");
}
