use std::fs::{File};
use std::io::{self, BufReader, Result};

//use itertools::Itertools;

/*

fn parse_line(ln : String) -> Vec<u64> {
    ln.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn crypt(key: Vec<u8>, cytxt: Vec<u8>) -> Vec<u8> {

}

*/


fn main() -> std::io::Result<()> {
    let filename = "p059_cipher.txt";
    
    println!("loading cyphertext {}...", filename);
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let cytxt : Vec<u8> = reader.read_to_string().split(",").map(|s| s.parse::<u8>().unwrap()).collect();
    
    println!("{:?}", cytxt);

    //for (i,ln) in lines.iter().enumerate() {
    //    println!("line {}, {}", i, ln);
    //}

    Ok(())
}