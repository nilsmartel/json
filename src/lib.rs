

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
    for c in iterator.filter(|c| !is_redundant(c)) {

    }
}

/// returns true, if character is redundant for parsing JSON data
/// @param c Character to be tested
fn is_redundant(c: char) -> bool {
    match c {
        '\n' | '\t' | ' ' => return true,
        _ => return false
    }
}
