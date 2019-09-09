use prettytable::{Cell, Row, Table};
use serde_json::Value;
use std::error;
use textwrap::fill;

pub struct Converter {}

impl Converter {
    pub fn kv_to_json(s: &String, delimiter: &str) -> Result<Value, Box<dyn error::Error>> {
        let lines = s.split(delimiter).collect::<Vec<&str>>();
        let args = lines
            .into_iter()
            .filter(|x| x.to_string() != "")
            .collect::<Vec<&str>>();
        let mut json_str = String::from("{");
        let mut aiter = args.iter().peekable();
        while let Some(i) = aiter.next() {
            let currarg = i.split(":").collect::<Vec<&str>>();
            if currarg.len() != 2 {
                Err(": is a delimiter and cannot part of a value in key-value pairs. Example \" key : valuehasa:somewhere \"")?
            } else {
                json_str.push_str(&format!("\"{}\":\"{}\"", currarg[0], currarg[1]));
            }
            if aiter.peek() == None {
                break;
            }
            json_str.push_str(",");
        }
        json_str.push_str("}");
        Ok(serde_json::from_str(&json_str).unwrap())
    }

    pub fn json_to_table(j: &Value) -> Result<Table, Box<dyn error::Error>> {
        let mut table = Table::new();
        for (k, v) in j.as_object().unwrap().iter() {
            if k == "src" || k == "dst" {
                continue;
            }
            table.add_row(Row::new(vec![
                Cell::new(&fill(&k.as_str(), 20)),
                Cell::new(&fill(&v.as_str().unwrap(), 20)),
            ]));
        }
        Ok(table)
    }
}
