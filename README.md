# Bex

Bex is a simple command line tool for converting hex numbers to binary and vice versa

## Download

1. you will need cargo to download from any method

### Crates.io 

download the package from crate.io using the cargo
`cargo install bex-rs`

### Build from source

1. git clone/download the repo
2. run `cargo build --release`
3. move the executable in "target/release" to your path
## Use
Type the base of the input and the number:
`bex [base] [number]`

examples :
 ```sh
~> bex 0x F025
0b 1111 0000 0010 0101
~> bex 0b 1100 0110
0x C6
```
 
