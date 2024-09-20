

fn main() {
    let args: Vec<String> = std::env::args().collect();
    const H_PREFIX: &str    = "0x";
    const B_PREFIX: &str    = "0b";
    const HELP_PARAM: &str  = "--help";
       

    match &args[1] {
         h if h == H_PREFIX => {
            print!("0b ");
            for n in args[2].chars() {
                print!("{} ",hex_to_bin(n))
            }
        },
        b if b == B_PREFIX => {
            print!("0x ");
            for nib in args.iter().skip(2) {
                print!("{}",bin_to_hex(nib));
            }
        },
        hlp if hlp == HELP_PARAM => {
            println!("/-------------BEX-------------\\");
            println!("| What is bex ?               |");
            println!("| Bex is a tool to convert    |");
            println!("| a hex number to binary a    |");
            println!("| number ands vice versa.     |");
            println!("| How to use ?                |");
            println!("| Type the type of the input  |");
            println!("| and the input number, like  |");
            println!("| this: bex 0x E030.          |");
            println!("| Modes :                     |");
            println!("|   Hex to Binary:            |");
            println!("|   ~> bex 0x F025            |");    
            println!("|   0b 1111 0000 0010 0101    |");
            println!("|   Binary to hex:            |");
            println!("|   ~> bex 0b 1110 0000 0010  |");
            println!("|   0x E02                    |");
            println!("\\-------------1.0-------------/");
        }
        _   => panic!("Error: invalid input type, Expected: 0x or 0b, Received: {}, type bex --help for help",args[1])
    }
}

fn hex_to_bin(hex: char) -> &'static str{
    return match hex {
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
    return match bin {
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
        _ => "????"
    }
}
