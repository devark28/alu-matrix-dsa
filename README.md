# Matrix DSA

A Rust implementation of sparse matrix operations including addition, subtraction, and multiplication.

## Overview

This project implements sparse matrix operations using a `HashMap`-based representation to efficiently store and manipulate large matrices with mostly zero elements.

### Supported Operations

*   Matrix Addition
*   Matrix Subtraction
*   Matrix Multiplication

## File Format

Matrices are stored in text files with the following format:

```
rows=<number_of_rows>
cols=<number_of_columns>
(row, column, value)
(row, column, value)
...
```

## Building the Project

1.  Install Rust and Cargo if not already installed:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  Clone the repository:

    ```bash
    git clone https://github.com/devark28/alu-matrix-dsa.git
    cd alu-matrix-dsa/dsa/sparse_matrix/code
    ```
3.  Build the project:

    ```bash
    cargo build --release
    ```

## Running the Program

The program takes 4 command line arguments:

```bash
./target/release/sparse_matrix_op <operation> <matrix1_file> <matrix2_file> <output_file>
```

Where:

*   `operation`: One of `add`, `sub`, or `mul`
*   `matrix1_file`: Path to first input matrix file
*   `matrix2_file`: Path to second input matrix file
*   `output_file`: Path where result matrix will be written

Example:

```bash
./target/release/sparse_matrix_op add ../sample_inputs/easy_sample_01_2.txt ../sample_inputs/easy_sample_01_3.txt result.txt
```

## Implementation Details

The project uses:

*   `HashMap<(i64, i64), i64>` to store non-zero matrix elements
*   Trait implementations for `Add`, `Sub`, and `Mul` operations
*   File I/O for reading matrix definitions and writing results
*   Error handling for invalid inputs and operations
