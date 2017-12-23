
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
    String(String)
}

pub fn parse(data: &str) -> JSON {
    let (rest, json) = parse_json(skip_redundant_characters(data.as_bytes()));
    json
}

fn parse_json(data: &[u8]) -> (&[u8], JSON) {
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
                if data[0] == ',' as u8 {
                    data = skip_redundant_characters([1..]);
                } else {
                    break;
                }
            }

            if data[0] != '}' as u8 {
                panic!("expected character '}'\n");
            }

            return(
                skip_redundant_characters(data[1..]),
                JSON::Object(map)
            );
        },
        '"' as u8 => {
            //  JSON::String
            let mut i = 1;
            while data[i] != '"' as u8 {
                i+=1;
            }

            return (
                skip_redundant_characters([i+1..]),
                JSON::String(String::From(data[1..i]))
            );
        },
        't' as u8 | 'f' as u8 => {
            //  JSON::Boolean
            if data[3] == 'e' as u8 {
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
        'n' as u8 => {
            //  JSON::Null
            return (
                skip_redundant_characters(data[4..]),
                JSON::Null
            );
        },
        '0' as u8 ... '9' as u8 | '+' as u8 | '-' as u8 => {
            let mut value_int: i64 = 0;
            let mut value_float: f64 = 0.0;
            let mut i = 0;
            let is_negative = if data[0] == '-' as u8 {true} else {false};
            if data[i] == '+' as u8 || is_negative {
                i=1;
            }

            while is_number(data[i]) {
                value_int = value_int*10 + (data[i]-'0') as i64;
                i+=1;
            }
            if data[i] == '.' as u8 {
                let mut j = 1;
                while is_number(data[i+j]) {
                    value_float += (data[i+j] - '0') as f64 / 10.powi(j)
                }

                return (
                    skip_redundant_characters(data[i+j..]),
                    JSON::Float(value_int as f64 + value_float)
                );
            }
            return (
                skip_redundant_characters(data[i..]),
                JSON::Integer(value_int)
            );
        },
    };
}

fn is_number(c: u8) -> bool {
    match c {
        '0' as u8 ... '9' as u8 => return true,
        _ => return false
    }
}

fn parse_till_name(text: &[u8]) -> (&[u8], String) {
    let mut i=0;
    while data[i] != '"' as u8 {
        i+=1;
    }
    i+=1;
    let mut name = "";
    while data[i] != '"' as u8 {
        name += data[i];
        i+=1;
    }
    while data[i] != ':' as u8 {
        i+=1;
    }

    (text[i+1..], String::from(name))
}

/// Skips all characters (redundant for JSON data)
/// at the beginning of a &[u8]
/// @param text JSON Text to be checked
/// returns JSON Text, with all redundant characters at the beginning skipped
fn skip_redundant_characters(text: &[u8]) -> &[u8] {
    let mut i: usize = 0;
    loop {
        if i < text.len() {
            if is_redundant(text[i]) {
                i+=1;
                continue;
            }
        }
        break;
    }

    str[i..]
}

/// Use it to test if a Character is of importance, when parsing JSON
/// @param c Character to be tested
/// returns true, if Character is redundant for JSON parsing
fn is_redundant(c: u8) -> bool {
    match c {
        '\n' as u8 | '\t' as u8 | ' ' as u8 => return true,
        _ => return false,
    }
}