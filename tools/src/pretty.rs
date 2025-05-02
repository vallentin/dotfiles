use std::error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use serde::Serialize;
use serde_json::{Serializer, ser::PrettyFormatter};

pub fn json() -> Result<(), Box<dyn error::Error>> {
    let mut json = String::new();
    input()?.read_to_string(&mut json)?;

    let data = serde_json::from_str::<serde_json::Value>(&json)?;

    let writer = io::stdout().lock();

    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(writer, fmt);
    data.serialize(&mut ser)?;

    println!();

    Ok(())
}

fn input() -> Result<Box<dyn BufRead>, Box<dyn error::Error>> {
    match std::env::args().nth(1) {
        Some(path) => {
            let file = File::open(path)?;
            let file = BufReader::new(file);
            Ok(Box::new(file))
        }
        None => {
            let stdin = io::stdin().lock();
            Ok(Box::new(stdin))
        }
    }
}
