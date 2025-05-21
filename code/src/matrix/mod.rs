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
}