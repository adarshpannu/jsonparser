// JSON Parser (as if the world needs another one)

#![allow(warnings)]

use std::cell::RefCell;

pub struct JSONParser<'a> {
    tokens: RefCell<Vec<&'a str>>,
    token_ix: RefCell<usize>,
}

#[derive(Debug)]
enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    StringLiteral(&'a str),
    NumericLiteral(&'a str),
    True,
    False,
    Null,
}

#[derive(Debug)]
struct ParseError<'a> {
    token: &'a str,
}

impl<'a> ParseError<'a> {
    fn new(token: &'a str) -> ParseError {
        ParseError { token }
    }
}

impl<'a> JSONParser<'a> {
    fn new(s: &str) -> JSONParser {
        let tokens = RefCell::new(Self::tokenize(s));
        dbg!(&tokens);
        JSONParser {
            tokens,
            token_ix: RefCell::new(0),
        }
    }

    fn parse(&self) -> Result<JSONValue, ParseError> {
        self.parseValue()
    }

    fn tokenize(s: &str) -> Vec<&str> {
        s.split(|c| c == ' ' || c == '\n')
            .flat_map(|elem| {
                if elem.len() > 1 && (elem.ends_with(",") || elem.ends_with(":")) {
                    let len = elem.len();
                    let v = vec![&elem[..len - 1], &elem[len - 1..]];
                    v
                } else {
                    vec![elem]
                }
            })
            .filter(|&s| s.len() > 0)
            .collect::<Vec<&str>>()
    }

    fn next_token(&self) -> Option<&str> {
        let mut token_ix = self.token_ix.borrow_mut();
        let tokens = self.tokens.borrow_mut();

        if *token_ix < tokens.len() {
            let token = tokens[*token_ix];
            *token_ix += 1;
            println!("NEXT TOKEN: {:?}", token);
            Some(token)
        } else {
            None
        }
    }

    fn unread_token(&self) {
        let mut token_ix = self.token_ix.borrow_mut();
        let tokens = self.tokens.borrow_mut();

        if *token_ix > 0 {
            *token_ix -= 1;
            let token = tokens[*token_ix];
            println!("UNREAD TOKEN: {:?}", token);
        } else {
            panic!("Internal error: Already at beginning of token stream, cannot unread another token.")
        }
    }

    fn peek_token(&self) -> Option<&str> {
        let token = self.next_token();
        self.unread_token();
        token
    }

    fn is_string(s: &str) -> bool {
        return s.len() > 0 && s.starts_with("\"") && s.ends_with("\"");
    }

    fn is_number(s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }

    fn parseValue(&self) -> Result<JSONValue, ParseError> {
        let token = self.peek_token();
        let token = token.unwrap();

        match token {
            "{" => self.parseObject(),
            "[" => self.parseArray(),
            "\"true\"" => self.parseTrue(),
            "\"false\"" => self.parseFalse(),
            "\"null\"" => self.parseNull(),
            _ => {
                if Self::is_string(token) {
                    self.parseString()
                } else if Self::is_number(token) {
                    self.parseNumber()
                } else {
                    Err(ParseError::new(token))
                }
            }
        }
    }

    fn parseArray(&self) -> Result<JSONValue, ParseError> {
        let mut array: Vec<JSONValue> = Vec::new();
        self.next_token(); // Eat "["

        while let Some(token) = self.next_token() {
            if token == "]" {
                return Ok(JSONValue::Array(array));
            }
            if array.len() > 0 {
                // Read comma (",")
                if token != "," {
                    return Err(ParseError::new("Missing comma"));
                }
            } else {
                self.unread_token();
            }
            // Read array element
            let jv = self.parseValue()?;
            array.push(jv);
        }
        return Err(ParseError::new("Incomplete input"));
    }

    fn parseObject(&self) -> Result<JSONValue, ParseError> {
        let mut attrs: Vec<(&str, JSONValue)> = Vec::new();
        self.next_token(); // Eat "["

        while let Some(token) = self.next_token() {
            if token == "}" {
                return return Ok(JSONValue::Object(attrs));
            }
            if attrs.len() > 0 {
                // Read comma (",")
                if token != "," {
                    return Err(ParseError::new("Missing comma"));
                }
            } else {
                self.unread_token();
            }
            // Read name:attr pair
            let name = self.parseValue()?;
            let name = match name {
                JSONValue::StringLiteral(name) => name,
                _ => return Err(ParseError::new("xxx")),
            };

            let colon = self.next_token();
            if colon != Some(":") {
                return Err(ParseError::new(": not found"));
            }
            let attr = self.parseValue()?;

            attrs.push((name, attr));
        }
        return Err(ParseError::new("Incomplete input"));
    }

    fn parseTrue(&self) -> Result<JSONValue, ParseError> {
        self.next_token();
        Ok(JSONValue::True)
    }

    fn parseFalse(&self) -> Result<JSONValue, ParseError> {
        self.next_token();
        Ok(JSONValue::False)
    }

    fn parseNull(&self) -> Result<JSONValue, ParseError> {
        self.next_token();
        Ok(JSONValue::Null)
    }

    fn parseString(&self) -> Result<JSONValue, ParseError> {
        let token = self.next_token().unwrap();
        Ok(JSONValue::StringLiteral(token))
    }

    fn parseNumber(&self) -> Result<JSONValue, ParseError> {
        let token = self.next_token().unwrap();
        Ok(JSONValue::NumericLiteral(token))
    }
}

#[test]
fn it_works() {
    use JSONParser;

    let jsonstr = r#"
    {
        name: "adarsh",
        age: 53
    }"#;

    let jsonstr = r#"
    [ "true" , "false" , [ "null" ] , "adarsh" , 1.32 ]
    "#;

    let jsonstr = r#"
    [ 1 , 2, [ 3 ] ]
    "#;

    let jsonstr = r#"
    { "hello" : "world" ,
      "red" : 1.0  , 
       "ages" : [ 45 , 65.7e6 ] , 
      "person" : { 
          "name" : "adarsh"
      }
     }
    "#;

    let mut jp = JSONParser::new(jsonstr);
    dbg!(jp.parse());
}

struct Tokenizer<'a> {
    str: &'a str,
}

fn tokenize(str: &str) -> Vec<&str> {
    let mut retvec: Vec<&str> = Vec::new();

    for (ix, ch) in str.chars().enumerate() {
        match ch {
            '{' | '}' | ',' | ':' => retvec.push(&str[ix..ix + 1]),
            '"' => retvec.push(&str[ix..ix + 1]),
            '-' | '+' | '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                retvec.push(&str[ix..ix + 1])
            }
            _ => retvec.push(&str[ix..ix + 1]),
        }
    }
    retvec
}

#[test]
fn test() {}
