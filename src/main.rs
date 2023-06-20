use std::io::{BufReader, BufRead, Result};

use epnl::{PNL, SPL};

fn main() {
    let spls = read_file("input.txt").expect("unable to read file");
    let spls: Vec<_> = spls.iter().map(|spl| SPL::from(&spl[..])).collect();
    println!("{spls:?}");
    let result = PNL::from(spls[1].clone());
    println!("{result:?}");       
    
}

fn read_file(filename: &str) -> Result<Vec<Vec<f64>>> {
    let file = std::fs::File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(|x| {process_line(x.ok()?)}).collect())
}

fn process_line(line: String) -> Option<Vec<f64>> {
    let data: Vec<f64> = line.split_whitespace()
        .filter_map(|x| {x.parse::<f64>().ok()}).collect();
    if data.is_empty() {None} else {Some(data)}
}


