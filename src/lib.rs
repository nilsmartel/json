
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let json_data: &str = "{\"name\":\"Nils\",\"age\":20,\"height\":1.83,\"programmer\":true}";
        let data: JSON = parse_json(json_data);
    }
}

enum JSON {
    Object(HashMap<String, JSON>),
    Array(Vec<JSON>),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String)
}

fn parse_json(data: &str) -> JSON { JSON::Integer(0) }
