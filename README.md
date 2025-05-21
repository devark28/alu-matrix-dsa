# Matrix DSA

A Rust implementation of sparse matrix operations including addition, subtraction, and multiplication.

## Overview

This project implements sparse matrix operations using a `HashMap`-based representation to efficiently store and manipulate large matrices with mostly zero elements.

**Supported Operations**

- Matrix Addition
- Matrix Subtraction
- Matrix Multiplication

## File Format

Matrices are stored in text files with the following format:

```
rows=<number_of_rows>
cols=<number_of_columns>
(row, column, value)
(row, column, value)
...
```

## Installation

Matrix DSA can be built on Linux, macOS, or Windows. Follow the steps below for your platform.

**1. Install Rust and Cargo**

- **Linux and macOS:**  
  Open a terminal and run:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  Follow the on-screen instructions to complete the installation. Then, restart your terminal or run:
  ```bash
  source ~/.cargo/env
  ```

- **Windows:**  
  Download and run `rustup-init.exe` from the official Rust website. Follow the installation prompts.

After installation, restart your terminal (or Command Prompt/PowerShell on Windows) to ensure the `cargo` command is available.

**2. Clone the Repository**

```bash
git clone https://github.com/devark28/alu-matrix-dsa.git
cd alu-matrix-dsa/dsa/sparse_matrix/code
```

**3. Build the Project**

```bash
cargo build --release
```

The compiled binary will be located at `./target/release/matrix-dsa` (or `.\target\release\matrix-dsa.exe` on Windows).

## Running the Program

The program takes 4 command line arguments:

```bash
./target/release/matrix-dsa    
```

Where:

- `operation`: One of `add`, `sub`, or `mul`
- `matrix1_file`: Path to first input matrix file
- `matrix2_file`: Path to second input matrix file
- `output_file`: Path where result matrix will be written

**Example:**

```bash
./target/release/matrix-dsa add ../sample_inputs/easy_sample_01_2.txt ../sample_inputs/easy_sample_01_3.txt result.txt
```

(On Windows, use `matrix-dsa.exe` instead of `matrix-dsa`.)

## Implementation Details

- Uses `HashMap` to store non-zero matrix elements
- Implements traits for `Add`, `Sub`, and `Mul` operations
- Handles file I/O for reading matrix definitions and writing results
- Provides error handling for invalid inputs and operations
