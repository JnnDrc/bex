
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut pipe_mode = false;
    let mut prefix = "";
    let mut counted = 1;
    const MANUAL: &str = 
r#"/-------------BEX-------------\
| What is bex ?               |
| Bex is a tool to convert    |
| a hex number to binary a    |
| number and vice versa.      |
|-----------------------------|
| How to use ?                |
| Type the type of the input  |
| and the input number, like  |
| this: bex 0x E030.          |
| Modes :                     |
|   Hex to Binary:            |
|   ~> bex 0x F025            |
|   0b 1111 0000 0010 0101    |
|   Binary to hex:            |
|   ~> bex 0b 1110 0000 0010  |
|   0x E02                    |
|-----------------------------|
| Arguments manual:           |
|   -h/--help: shows this     |
| manual                      |
|   -p/--pipe: remove the type|
|of the output(0x/0b) to easi-|
|ly pipe the output           |
\\-----------------------0.1.1/"#;
    
    if args.len() < 2{
        panic!("Error: missing inputs, Expected: 0x or 0b, Received: nothing, type bex --help for help")
    }
    if &args[1] == "--help" || &args[1] == "-h"{
        print!("{MANUAL}");
        return;
    }
    for arg in &args[1..]{
        if  arg == "-p" || arg == "--pipe"{
            pipe_mode = true;
            counted+=1;
        }else if arg == "0x" || arg == "0b"{
            prefix = arg;
            counted+=1;
            break;
        }
    }

    match prefix {
         "0x" => {
             if !pipe_mode{
                 print!("0b ");
             }
            for n in args[counted].chars() {
                print!("{} ",hex_to_bin(n))
            }
        },
        "0b" => {
            if !pipe_mode{
                print!("0x ");
            }
            for nib in args.iter().skip(counted) {
                print!("{}",bin_to_hex(nib));
            }
        }
        _   => panic!("Error: invalid input type, Expected: 0x or 0b, Received: {}, type bex --help for help",prefix)
    }
}

fn hex_to_bin(hex: char) -> &'static str{
    match hex {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => "????"
    }
}

fn bin_to_hex(bin: & str) -> &str{
    match bin {
        "0000" => "0",
        "0001" => "1",
        "0010" => "2",
        "0011" => "3",
        "0100" => "4",
        "0101" => "5",
        "0110" => "6",
        "0111" => "7",
        "1000" => "8",
        "1001" => "9",
        "1010" => "A",
        "1011" => "B",
        "1100" => "C",
        "1101" => "D",
        "1110" => "E",
        "1111" => "F",
        _ => "?"
    }
}
