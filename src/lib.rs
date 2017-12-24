

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let json_data: &str = "{\"name\":\"Nils\",\"age\":20,\"height\":1.83,\"programmer\":true}";
        let data = parse(json_data);
    }

    #[test]
    fn test_redundant() {
        let data: &str = "{\n  \"name\":\"Nils\",\n  \"age\":20,\n  \"height\":1.83,\n  \"programmer\":true\n}\n";
        let mut result: String= String::new();
        for c in data.chars().filter(|c| !is_redundant(c)) {
            result.push(c);
        }

        assert_eq!(result.as_str(), "{\"name\":\"Nils\",\"age\":20,\"height\":1.83,\"programmer\":true}");
    }
}

/// Data Structure to represent all values and types,
/// that might be nestes inside a JSON-Textfile
///
// TODO So far I ignored hashmaps, bad Nils, bad!
pub enum JSON {
    Null,
    Object(HashMap<String, JSON>),
    Array(Vec<JSON>),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

/// Parses JSON Data (in form of &str)
/// @param data JSON Data(in form of String) that needs to be parsed
pub fn parse(data: &str) -> JSON {
    for c in data.chars().filter(|c| !is_redundant(c)) {
    }

    JSON::Null
}

/// returns true, if character is redundant for parsing JSON data
/// @param c Character to be tested
fn is_redundant(c: &char) -> bool {
    match *c {
        '\n' | '\t' | ' ' => return true,
        _ => return false
    }
}
