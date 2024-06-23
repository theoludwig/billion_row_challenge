<h1 align="center">theoludwig/billion_row_challenge</h1>

<p align="center">
  <strong>My Solution for the <a href="https://1brc.dev/">1 Billion Row Challenge</a>, implemented in the <a href="https://www.rust-lang.org/">Rust Programming Language</a>.</strong>
</p>

<p align="center">
  <a href="https://github.com/theoludwig/billion_row_challenge/actions/workflows/ci.yml"><img src="https://github.com/theoludwig/billion_row_challenge/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust%20MSRV-v1.79.0-blue?logo=rust" alt="Rust" /></a>
  <a href="https://conventionalcommits.org"><img src="https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg" alt="Conventional Commits" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/licence-MIT-blue.svg" alt="Licence MIT"/></a>
</p>

## About

1️⃣🐝🏎️ The One Billion Row Challenge (1BRC) is a fun exploration of how far modern programming languages (initally only Java) can be pushed to **calculate** the **min, max, and average of 1 billion measurements** as fast as possible.

The repository contains **my solution** for the [1BRC](https://1brc.dev/) challenge, implemented in the [Rust programming language](https://www.rust-lang.org/).

![1BRC](./1brc.png)

### Links

- <https://github.com/gunnarmorling/1brc>
- <https://www.morling.dev/blog/one-billion-row-challenge/>
- <https://1brc.dev/>

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) >= v1.79.0
- [Java](https://openjdk.org/) v21 (used to generate the 1 billion row data)

### Installation

```sh
# Clone the repository
git clone git@github.com:theoludwig/billion_row_challenge.git

# Go to the project root
cd billion_row_challenge

# Rust related commands
cargo run
cargo build --release
cargo test
cargo clippy --verbose -- -D warnings
cargo fmt -- --check
```

### Usage

```sh
# Build (optimized)
cargo build --release

# Usage: ./target/release/billion_row_challenge <input_file_path>

# Example with fixture data
./target/release/billion_row_challenge ./tests/fixtures/10/input.txt

# Example with the 1 billion row data (not included in the repository, needs to be generated)
./target/release/billion_row_challenge ./1brc/measurements.txt
```

### Generate the 1 Billion Row Data (~12GB)

```sh
# Clone the 1brc repository
git clone git@github.com:gunnarmorling/1brc.git

# Go to the project root
cd 1brc

# Build the project using Apache Maven
./mvnw clean verify

# Create the `measurements.txt` file with 1B rows
./create_measurements.sh 1000000000
```

## Challenge Instructions

The text file contains temperature values for a range of weather stations. Each row is one measurement in the format `<string: station name>;<double: measurement>`, with the measurement value having exactly one fractional digit. The following shows ten rows as an example:

```txt
Hamburg;12.0
Bulawayo;8.9
Palembang;38.8
St. John's;15.2
Cracow;12.6
Bridgetown;26.9
Istanbul;6.2
Roseau;34.4
Conakry;31.2
Istanbul;23.0
```

The task is to write a program which reads the file, calculates the **min**, **mean**, and **max** temperature value **per weather station**, and emits the results on stdout like this (i.e. sorted alphabetically by station name, and the result values per station in the format `<min>/<mean>/<max>`, rounded to one fractional digit):

```txt
{Abha=-23.0/18.0/59.2, Abidjan=-16.2/26.0/67.3, Abéché=-10.0/29.4/69.0, Accra=-10.1/26.4/66.4, Addis Ababa=-23.7/16.0/67.0, Adelaide=-27.8/17.3/58.5, ...}
```

### Limits

- Input value ranges are as follows:
  - **Station name:** non null UTF-8 string of min length 1 character and max length 100 bytes, containing neither `;` nor `\n` characters. (i.e. this could be 100 one-byte characters, or 50 two-byte characters, etc.).
  - **Temperature value:** non null double between -99.9 (inclusive) and 99.9 (inclusive), always with one fractional digit.
- There is a maximum of $10,000$ unique station names.
- Line endings in the file are `\n` characters on all platforms.
- The rounding of output values must be done using the semantics of IEEE 754 rounding-direction "roundTowardPositive".

### Examples

See the  [`tests/fixtures`](./tests/fixtures) folder for examples of input/output.

#### Input

```txt
Halifax;12.9
Zagreb;12.2
Cabo San Lucas;14.9
Adelaide;15.0
Ségou;25.7
Pittsburgh;9.7
Karachi;15.4
Xi'an;24.2
Dodoma;22.2
Tauranga;38.2

```

#### Output

```txt
{Adelaide=15.0/15.0/15.0, Cabo San Lucas=14.9/14.9/14.9, Dodoma=22.2/22.2/22.2, Halifax=12.9/12.9/12.9, Karachi=15.4/15.4/15.4, Pittsburgh=9.7/9.7/9.7, Ségou=25.7/25.7/25.7, Tauranga=38.2/38.2/38.2, Xi'an=24.2/24.2/24.2, Zagreb=12.2/12.2/12.2}
```

## License

[MIT](./LICENSE)
