
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

fn parse_json(data: &str) -> JSON {
    let data = skip_redundant_characters(data);
}

fn skip_redundant_characters(text: &str) -> &str {
    let mut i = 0;
    while is_redundant(str[i]) {
        i+=1;
    }

    str[i:]
}

/// is_redundant
/// @param c Character to be tested
/// returns true, if Character is redundant for JSON parsing
fn is_redundant(c: char) -> bool {
    match c {
        '\n' | '\t' | ' ' => return true,
        _ => return false,
    }
}