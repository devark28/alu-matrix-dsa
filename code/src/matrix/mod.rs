use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::ops::{Add, Mul, Sub};
use std::process::exit;

#[derive(Debug, Clone)]
pub struct SparseMatrix {
    pub rows: i64,
    pub cols: i64,
    matrix: HashMap<(i64, i64), i64>,
}

impl SparseMatrix {
    pub fn new(rows: i64, cols: i64) -> SparseMatrix {
        SparseMatrix {
            rows,
            cols,
            matrix: HashMap::new(),
        }
    }

    pub fn create_from_file(file_name: String) -> SparseMatrix {
        let mut matrix: HashMap<(i64, i64), i64> = HashMap::new();
        let file = match File::open(&file_name) {
            Ok(file) => file,
            Err(why) => {
                println!("Couldn't open {}: {}", &file_name, why);
                exit(1);
            },
        };
        let mut rows = 0;
        let mut cols = 0;
        let buffer = BufReader::new(file);
        for line in buffer.lines() {
            let line = line.unwrap();
            if line.clone().trim().starts_with("rows") {
                rows = line
                    .to_owned()
                    .split('=')
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
            } else if line.clone().trim().starts_with("cols") {
                cols = line
                    .to_owned()
                    .split('=')
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
            }
            let line = line.trim().chars().collect::<Vec<char>>();
            if line.first().unwrap().to_owned() == '(' && line.last().unwrap().to_owned() == ')' {
                let line = line.iter().skip(1).take(line.len() - 2).collect::<String>();
                let line = line
                    .split(',')
                    .map(|x| x.trim().parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                if line.len() == 3 {
                    matrix.insert((line[0], line[1]), line[2]);
                }
            }
        }
        SparseMatrix { matrix, rows, cols }
    }

    pub fn write_to_file(&self, file_name: String) {
        let mut file = match File::create(&file_name) {
            Ok(file) => file,
            Err(why) => {
                println!("Couldn't create {}: {}", &file_name, why);
                exit(1);
            },
        };
        let mut contents = String::new();
        contents.push_str(&format!("rows={}\n", self.rows));
        contents.push_str(&format!("cols={}\n", self.cols));
        for ((x, y), value) in &self.matrix {
            contents.push_str(&format!("({}, {}, {})\n", x, y, value));
        }
        contents.push_str(&format!("({})", self.rows * self.cols));
        file.write_all(contents.as_bytes()).unwrap();
    }

    fn peek_value(&self, row: i64, col: i64) -> i64 {
        match self.matrix.get(&(row, col)) {
            Some(value) => *value,
            None if row <= self.rows && col <= self.cols => 0,
            None => {
                println!("Matrix value not found");
                exit(1)
            },
        }
    }

    fn consume_value(&mut self, row: i64, col: i64) -> i64 {
        match self.matrix.remove(&(row, col)) {
            Some(value) => value,
            None if row <= self.rows && col <= self.cols => 0,
            None => {
                println!("Matrix value not found");
                exit(1)
            },
        }
    }

    fn add_to_value(&mut self, row: i64, col: i64, value: i64) {
        self.matrix.insert((row, col), self.peek_value(row, col) + value);
    }
}

impl Add for SparseMatrix {
    type Output = SparseMatrix;
    fn add(mut self, mut other: SparseMatrix) -> SparseMatrix {
        if self.rows != other.rows || self.cols != other.cols {
            println!("Matrix dimension mismatch");
            exit(1);
        }
        let mut matrix: HashMap<(i64, i64), i64> = HashMap::new();
        for ((x, y), value) in &self.matrix {
            matrix.insert((*x, *y), value + other.consume_value(*x, *y));
        }
        for ((x, y), value) in &other.matrix {
            matrix.insert((*x, *y), value + self.consume_value(*x, *y));
        }
        SparseMatrix {
            matrix,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Sub for SparseMatrix {
    type Output = SparseMatrix;
    fn sub(mut self, mut other: SparseMatrix) -> SparseMatrix {
        if self.rows != other.rows || self.cols != other.cols {
            println!("Matrix dimension mismatch");
            exit(1);
        }
        let mut matrix: HashMap<(i64, i64), i64> = HashMap::new();
        for ((x, y), value) in &self.matrix {
            matrix.insert((*x, *y), value + other.consume_value(*x, *y));
        }
        for ((x, y), value) in &other.matrix {
            matrix.insert((*x, *y), value + self.consume_value(*x, *y));
        }
        SparseMatrix {
            matrix,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Mul for SparseMatrix {
    type Output = SparseMatrix;
    fn mul(self, other: SparseMatrix) -> SparseMatrix {
        if self.cols != other.rows {
            println!("Matrix dimension mismatch");
            exit(1);
        }
        // let mut matrix: HashMap<(i64, i64), i64> = HashMap::new();
        // for ((a, _b), value) in &self.matrix {
        //     for ((_x, y), value2) in &other.matrix {
        //         matrix.insert(
        //             (*a, *y),
        //             matrix.get(&(*a, *y)).unwrap_or(&0) + value * value2,
        //         );
        //     }
        // }
        // SparseMatrix {
        //     matrix,
        //     rows: self.rows,
        //     cols: other.cols,
        // }
        let mut matrix: SparseMatrix = SparseMatrix::new(self.rows, other.cols);
        for ((a, b), value) in &self.matrix {
            for ((x, y), value2) in &other.matrix {
                if b != x {
                    break;
                }
                matrix.add_to_value(*a, *y, value * value2);
            }
        }
        matrix
    }
}

// impl Mul for SparseMatrix {
//     type Output = SparseMatrix;
//     fn mul(self, other: SparseMatrix) -> SparseMatrix {
//         if self.cols != other.rows {
//             println!("Matrix dimension mismatch");
//             exit(1);
//         }
//         let mut matrix: HashMap<(i64, i64), i64> = HashMap::new();
//
//         for i in 1..=self.rows {
//             for k in 1..=other.cols {
//                 let mut sum = 0;
//                 // Check all elements where self's column == other's row
//                 for j in 1..=self.cols {
//                     sum += self.peek_value(i, j) * other.peek_value(j, k);
//                 }
//                 if sum != 0 {
//                     matrix.insert((i, k), sum);
//                 }
//             }
//         }
//
//         SparseMatrix {
//             matrix,
//             rows: self.rows,
//             cols: other.cols,
//         }
//     }
// }
