// Checks if setting x at sudoku[i][j] is possible
fn check(sudoku: &[[u8; 9]; 9], i: usize, j: usize, x: u8) -> bool {
    // Check column and line
    for k in 0..9 {
        if sudoku[i][k] == x || sudoku[k][j] == x {
            return false;
        }
    }

    // Check tile
    let x_i = i / 3;
    let x_j = j / 3;
    for k in 0..3 {
        for n in 0..3 {
            if sudoku[n + 3 * x_i][k + 3 * x_j] == x {
                return false;
            }
        }
    }

    return true;
}

fn is_full(sudoku: &[[u8; 9]; 9]) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j] == 0 {
                return false;
            }
        }
    }
    return true;
}

fn solve(sudoku: &mut [[u8; 9]; 9]) -> bool {
    

    fn backtrack(sudoku: &mut [[u8; 9]; 9], mut i: usize, mut j:  usize) -> bool{
        if j > 8 {
            j = 0;
            i += 1;
            if i > 8 && is_full(sudoku) {
                print_sudoku(sudoku);
                return true;
            }
        }

        if sudoku[i][j] != 0 {
            if backtrack(sudoku, i, j+1) {
                return true;
            }
        }
        else {
            for k in 1..10 {
                if check(sudoku, i, j, k) {
                    sudoku[i][j] = k;
                    if backtrack(sudoku, i, j+1) {
                        return true;
                    }
                    sudoku[i][j] = 0;
                }
            }
        }
    return false;
    }
    return backtrack(sudoku, 0, 0);
}

fn print_sudoku(sudoku: &[[u8; 9]; 9]) {
    for i in 0..9 {
        if i != 0 && i % 3 == 0 {
            println!("------+-------+------");
        }
        for j in 0..9 {
            if j != 0 && j % 3 == 0 {
                print!("| ");
            }
            print!("{} ", sudoku[i][j]);
            if j == 8 {
                print!("\n");
            }
        }
    }
}

fn main() {
    let mut s: [[u8; 9]; 9] = [
        [0, 0, 0, 2, 6, 0, 7, 0, 1],
        [6, 8, 0, 0, 7, 0, 0, 9, 0],
        [1, 9, 0, 0, 0, 4, 5, 0, 0],
        [8, 2, 0, 1, 0, 0, 0, 4, 0],
        [0, 0, 4, 6, 0, 2, 9, 0, 0],
        [0, 5, 0, 0, 0, 3, 0, 2, 8],
        [0, 0, 9, 3, 0, 0, 0, 7, 4],
        [0, 4, 0, 0, 5, 0, 0, 3, 6],
        [7, 0, 3, 0, 1, 8, 0, 0, 0]
    ];
    solve(&mut s);
}
