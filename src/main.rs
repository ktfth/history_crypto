mod lib;

use lib::{cipher_xor, decipher_xor};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 && args[1] == "cipher" {
        println!("{}", cipher_xor(&args[2], &args[3]));
    } else if args.len() == 4 && args[1] == "decipher" {
        println!("{}", decipher_xor(&args[2], &args[3]));
    } else {
        println!("Options available \"cipher\" and \"decipher\".");
    }
}
