use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
    pub kv: Value,
}

impl Node {
    pub fn to_string(&self) -> String {
        let ret = String::from("{");
        let unend = self
            .kv
            .as_object()
            .unwrap()
            .iter()
            .fold(ret, |acc, (k, v)| {
                format!(
                    "{}\"{}\":\"{}\",",
                    acc,
                    k.trim(),
                    v.as_str().unwrap().trim()
                )
            });
        unend[0..unend.len() - 1].to_string() + "}"
    }
}
