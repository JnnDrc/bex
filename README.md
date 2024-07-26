# Bex

Bex is a simple command line tool for converting hex numbers to binary and vice versa

## Download

1. get the binary:
  * for windows users : download the .exe in target release
  * for linux/macOS users : download/git clone the project and run cargo build --release on it
2. add the folder it's in the $PATH variable or move it to a folder that's in $PATH

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
 
