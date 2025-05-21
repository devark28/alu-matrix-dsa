use std::env::args;
use std::process::exit;

pub enum Op {
    Add(char),
    Sub(char),
    Mul(char)
}

pub fn extract_args() -> (Op, String, String, String) {
    let op = args().collect::<Vec<String>>()[1].to_owned();
    let matrix_file_1 = args().collect::<Vec<String>>()[2].to_owned();
    let matrix_file_2 = args().collect::<Vec<String>>()[3].to_owned();
    let matrix_file_3 = args().collect::<Vec<String>>()[4].to_owned();
    let op = match op.as_str() {
        "add" => Op::Add('+'),
        "sub" => Op::Sub('-'),
        "mul" => Op::Mul('*'),
        _ => {
            println!("Invalid operation");
            exit(1);
        }
    };
    (op, matrix_file_1, matrix_file_2, matrix_file_3)
}
