
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    #[test]
    fn it_works() {
        let json_data: &str = "{\"name\":\"Nils\",\"age\":20,\"height\":1.83,\"programmer\":true}";
        let data = parse(json_data);
    }

    #[test]
    fn skip_chars() {
        assert_eq!(
            skip_redundant_characters(" \n  hello".as_bytes()),
            "hello".as_bytes()
        );
    }
}

enum JSON {
    Null,
    Object(HashMap<String, JSON>),
    Array(Vec<JSON>),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

pub fn parse(data: &str) -> JSON {}
