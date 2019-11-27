use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Rel {
    pub id: String,
    pub kv: Value,
    pub src_id: String,
    pub dst_id: String,
}

impl Rel {
    pub fn to_string(&self) -> String {
        let ret = format!(
            "{{\"src\":{},\"dst\":{},\"kv\":{{",
            self.src_id, self.dst_id
        );
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
        unend[0..unend.len() - 1].to_string() + "}} }}"
    }
}
