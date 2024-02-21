# Sudoku-Solver

Small project : Sudoku solver implemented in Rust.

Everything is in `src/main.rs`. The solving method is backtracking : it fills the sudoku cell by cell, building a tree of all possibilities and backtrack if there is no solution in the current branch.
This only implements 9 by 9 grid.


run `cargo run` to compile and run the project.

(if you want to try your own grid, change `s` variable in `main` function.)
