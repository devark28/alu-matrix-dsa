mod matrix;
mod utils;

use utils::Op;
use matrix::SparseMatrix;
use std::env::args;
use std::process::exit;
use utils::extract_args;

fn main() {
    if args().collect::<Vec<String>>().len() != 5 {
        println!("Usage: ./sparse_matrix_op <add|sub|mul> <matrix_1> <matrix_2> <output>");
        exit(1)
    }

    let (op, matrix_file_1, matrix_file_2, matrix_file_3) = extract_args();

    let matrix_1: SparseMatrix = SparseMatrix::create_from_file(matrix_file_1);
    let matrix_2: SparseMatrix = SparseMatrix::create_from_file(matrix_file_2);

    println!(
        "{:?}x{:?} {} {:?}x{:?}",
        matrix_1.rows,
        matrix_1.cols,
        match op {
            Op::Add(c) => c,
            Op::Sub(c) => c,
            Op::Mul(c) => c,
        },
        matrix_2.rows,
        matrix_2.cols
    );
}
