// JSON Parser (as if the world needs another one)

#![allow(warnings)]

use std::cell::RefCell;

pub struct JSONParser<'a> {
    tokens: RefCell<Vec<&'a str>>,
    token_ix: RefCell<usize>,
}

#[derive(Debug)]
enum JSONValue<'a> {
    Object,
    Array(Vec<JSONValue<'a>>),
    StringLiteral(&'a str),
    NumericLiteral(&'a str),
    True,
    False,
    Null,
}

#[derive(Debug)]
struct ParseError;

impl<'a> JSONParser<'a> {
    fn new(s: &str) -> JSONParser {
        let tokens = RefCell::new(Self::tokenize(s));
        dbg!(&tokens);
        JSONParser {
            tokens,
            token_ix: RefCell::new(0)
        }
    }

    fn parse(&self) -> Result<JSONValue, ParseError> {
        self.parseValue()
    }

    fn tokenize(s: &str) -> Vec<&str> {
        s.split(|c| c == ' ' || c == '\n')
            .filter(|&s| s.len() > 0)
            .collect::<Vec<&str>>()
    }

    fn next_token(&self) -> Option<&str> {
        let mut token_ix = self.token_ix.borrow_mut();
        let tokens = self.tokens.borrow_mut();

        if *token_ix <= tokens.len() {
            let token = tokens[*token_ix];
            *token_ix += 1;
            Some(token)
        } else {
            None
        }
    }

    fn parseValue(&self) -> Result<JSONValue, ParseError> {
        let token = self.next_token();
        match token.unwrap() {
            "{" => self.parseObject(),
            "[" => self.parseArray(),
            "\"true\"" => self.parseTrue(),
            "\"false\"" => self.parseFalse(),
            "\"null\"" => self.parseNull(),
            _ => Err(ParseError),
        }
    }

    fn parseArray(&self) -> Result<JSONValue, ParseError> {
        let mut array: Vec<JSONValue> = Vec::new();

        loop {
            let token = self.next_token();
            match token {
                None => return Err(ParseError),
                Some("]") => return Ok(JSONValue::Array(array)),
                _ => {
                    let jv = self.parseValue()?;
                    array.push(jv);
                },
            }
        }
    }

    fn parseObject(&self) -> Result<JSONValue, ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseTrue(&self) -> Result<JSONValue, ParseError> {
        Ok(JSONValue::True)
    }

    fn parseFalse(&self) -> Result<JSONValue, ParseError> {
        Ok(JSONValue::False)
    }

    fn parseNull(&self) -> Result<JSONValue, ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseNameValuePairs(&self) -> Result<JSONValue, ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseNameValuePair(&self) -> Result<JSONValue, ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseName(&self) -> Result<JSONValue, ParseError> {
        Ok(JSONValue::Null)
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
    "true"
    "#;

    let mut jp = JSONParser::new(jsonstr);
    dbg!(jp.parse());
}
