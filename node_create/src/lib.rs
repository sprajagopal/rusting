use std::process::{Command, Child};
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::{Value, };
use std::error;

pub fn editor(fname : &String, editor : &String) -> Result<Child, Box<error::Error>> {
    let mut cmd = Command::new(editor)
        .arg(fname)
        .spawn()?;

    cmd.wait()?;
    Ok(cmd)
}

pub fn file_to_string(fname: &String) -> Result<String, Box<error::Error>> {
    let file = File::open(fname)?;
    let mut raw = String::new();
    raw.push_str("{");
    let mut iter = BufReader::new(file).lines().peekable(); 
    while let Some(Ok(line)) = iter.next() {
        let vec = line.split(":").collect::<Vec<&str>>();
        raw.push_str(&format!("\"{}\":\"{}\"", vec[0].trim(), vec[1].trim()));
        if let None = iter.peek() {
            break;
        } 

        raw.push_str(",");
    }
    raw.push_str("}");
    println!("  Formed json {}", raw);
    Ok(raw)
} 

pub fn file_to_dict(fname: &String) -> Result<Value, Box<error::Error>>{
   let raw = file_to_string(fname)?;
   let v = serde_json::from_str(&raw)?;
   Ok(v)
}

