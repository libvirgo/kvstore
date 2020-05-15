use std::io;
use std::fs::File;
use std::io::BufRead;

pub fn read_lines(path:&str) ->io::Result<io::Lines<io::BufReader<File>>>{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}