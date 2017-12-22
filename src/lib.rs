
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let json_data: &str = "{\"name\":\"Nils\",\"age\":20,\"height\":1.83,\"programmer\":true}";
        let data = parse(json_data);
    }
}

enum JSON {
    Null,
    Object(HashMap<String, JSON>),
    Array(Vec<JSON>),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String)
}

pub fn parse(data: &str) -> JSON {
    let (rest, json) = parse_json(skip_redundant_characters(data));
    json
}

fn parse_json(data: &str) -> (&str, JSON) {
    //  let mut data = skip_redundant_characters(data);

    let obj: JSON = match data[0] {
        '{' => {
            //  JSON::Object(Hashmap<String, JSON>)
            let mut data = data[..];
            let mut map: HashMap<String, JSON>;
            let mut name: String;
            let mut value: JSON;

            loop {
                (data, name) = parse_till_name(data[1..]);
                (data, value) = parse_json(data);
                map.insert(name, value);
                if data[0] == ',' {
                    data = skip_redundant_characters([1..]);
                } else {
                    break;
                }
            }

            if data[0] != '}' {
                panic!("expected character '}'\n");
            }

            return(
                skip_redundant_characters(data[1..]),
                JSON::Object(map)
            );
        },
        '"' => {
            //  JSON::String
            let mut i = 1;
            while data[i] != '"' {
                i+=1;
            }

            return (
                skip_redundant_characters([i+1..]),
                JSON::String(String::From(data[1..i]))
            );
        },
        't' | 'f' => {
            //  JSON::Boolean
            if data[3] == 'e' {
                return (
                    skip_redundant_characters([4..]),
                    JSON::Boolean(true)
                );
            }
            return (
                skip_redundant_characters(data[5..]),
                JSON::Boolean(false)
            );
        },
        'n' => {
            //  JSON::Null
            return (
                skip_redundant_characters(data[4..]),
                JSON::Null
            );
        },
        '0' ... '9' | '+' | '-' => {
            let mut value_int: i64 = 0;
            let mut value_float: f64 = 0.0;
            let mut i = 0;
            let is_negative = if data[0] == '-' {true} else {false};
            if data[i] == '+' || is_negative {
                i=1;
            }

            while is_number(data[i]) {
                value_int = value_int*10 + (data[i]-'0') as i64;
                i+=1;
            }
            if data[i] == '.' {

            }
            return (
                skip_redundant_characters(data[i..]),
                JSON::Integer(value_int)
            );
        },
    };
}

fn is_number(c: char) -> bool {
    match c {
        '0' ... '9' => return true,
        _ => return false
    }
}

fn parse_till_name(text: &str) -> (&str, String) {
    let mut i=0;
    while data[i] != '"' {
        i+=1;
    }
    i+=1;
    let mut name = "";
    while data[i] != '"' {
        name += data[i];
        i+=1;
    }
    while data[i] != ':' {
        i+=1;
    }

    (text[i+1..], String::from(name))
}

/// Skips all characters (redundant for JSON data)
/// at the beginning of a &str
/// @param text JSON Text to be checked
/// returns JSON Text, with all redundant characters at the beginning skipped
fn skip_redundant_characters(text: &str) -> &str {
    let mut i = 0;
    while is_redundant(str[i]) {
        i+=1;
    }

    str[i..]
}

/// Use it to test if a Character is of importance, when parsing JSON
/// @param c Character to be tested
/// returns true, if Character is redundant for JSON parsing
fn is_redundant(c: char) -> bool {
    match c {
        '\n' | '\t' | ' ' => return true,
        _ => return false,
    }
}