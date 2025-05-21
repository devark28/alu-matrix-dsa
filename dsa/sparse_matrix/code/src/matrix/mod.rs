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
            }
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
            }
        };
        let mut contents = String::new();
        contents.push_str(&format!("rows={}\n", self.rows));
        contents.push_str(&format!("cols={}\n", self.cols));
        for ((x, y), value) in &self.matrix {
            contents.push_str(&format!("({}, {}, {})\n", x, y, value));
        }
        file.write_all(contents.as_bytes()).unwrap();
    }

    fn peek_value(&self, row: i64, col: i64) -> Option<i64> {
        match self.matrix.get(&(row, col)) {
            Some(value) => Some(*value),
            None if row <= self.rows && col <= self.cols => None,
            None => {
                println!("Matrix value not found");
                exit(1)
            }
        }
    }

    fn consume_value(&mut self, row: i64, col: i64) -> Option<i64> {
        match self.matrix.remove(&(row, col)) {
            Some(value) => Some(value),
            None if row <= self.rows && col <= self.cols => None,
            None => {
                println!("Matrix value not found");
                exit(1)
            }
        }
    }

    fn add_to_value(&mut self, row: i64, col: i64, value: i64) {
        if let Some(peeked_value) = self.peek_value(row, col) {
            if peeked_value + value == 0 {
                self.matrix.remove(&(row, col));
            } else {
                self.matrix.insert((row, col), peeked_value + value);
            }
        } else if value != 0 {
            self.matrix.insert((row, col), value);
        }
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
            if let Some(consumed_value) = other.consume_value(*x, *y) {
                if value + consumed_value != 0 {
                    matrix.insert((*x, *y), value + consumed_value);
                }
            } else if *value != 0 {
                matrix.insert((*x, *y), *value);
            }
        }
        for ((x, y), value) in &other.matrix {
            if let Some(consumed_value) = self.consume_value(*x, *y) {
                if value + consumed_value != 0 {
                    matrix.insert((*x, *y), value + consumed_value);
                }
            } else if *value != 0 {
                matrix.insert((*x, *y), *value);
            }
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
            if let Some(consumed_value) = other.consume_value(*x, *y) {
                if value - consumed_value != 0 {
                    matrix.insert((*x, *y), value - consumed_value);
                }
            } else if *value != 0 {
                matrix.insert((*x, *y), *value);
            }
        }
        for ((x, y), value) in &other.matrix {
            if let Some(consumed_value) = self.consume_value(*x, *y) {
                if consumed_value - value != 0 {
                    matrix.insert((*x, *y), consumed_value - value);
                }
            } else if *value != 0 {
                matrix.insert((*x, *y), *value);
            }
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
        let group_other_by_row =
            other
                .matrix
                .iter()
                .fold(HashMap::new(), |mut acc, ((x, y), value)| {
                    acc.entry(*x).or_insert_with(Vec::new).push((*y, *value));
                    acc
                });
        let mut matrix: SparseMatrix = SparseMatrix::new(self.rows, other.cols);
        for ((a, b), value) in &self.matrix {
            if let Some(values) = group_other_by_row.get(b) {
                for (c, value2) in values {
                    matrix.add_to_value(*a, *c, value * value2);
                }
            }
        }
        matrix
    }
}
