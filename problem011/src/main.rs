use std::env;
use std::fs::{File};
use std::path::Path;
use std::io::{self, BufRead, Result};



fn parse_line(ln : String) -> Vec<u64> {
    ln.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()
}



fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("loading {}...", filename);
    
    let path = Path::new(filename);
    let display = path.display();

    let file = File::open(path)?;
    let lines : Vec<_> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();

    println!("{:?}", lines);

    //for (i,ln) in lines.iter().enumerate() {
    //    println!("line {}, {}", i, ln);
    //}

    Ok(())
}