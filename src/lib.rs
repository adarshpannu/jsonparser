// JSON Parser (as if the world needs another one)

#![allow(warnings)]

pub struct JSONParser<'a> {
    tokens: Vec<&'a str>,
    token_ix: usize,
}

#[derive(Debug)]
enum JSONValue<'a> {
    Object,
    Array,
    StringLiteral(&'a str),
    NumericLiteral(&'a str),
    True,
    False,
    Null
}

#[derive(Debug)]
struct ParseError;

impl<'a> JSONParser<'a> {
    fn new(s: &str) -> JSONParser {
        let tokens = Self::tokenize(s);
        dbg!(&tokens);
        JSONParser {
            tokens,
            token_ix: 0,
        }
    }

    fn parse(&mut self) -> Result<JSONValue,ParseError> {
        self.parseValue()
    }

    fn tokenize(s: &str) -> Vec<&str> {
        s.split(|c| c == ' ' || c == '\n')
            .filter(|&s| s.len() > 0)
            .collect::<Vec<&str>>()
    }

    fn next_token(&mut self) -> Option<&str> {
        if self.token_ix <= self.tokens.len() {
            let token = self.tokens[self.token_ix];
            self.token_ix += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek_token(&mut self) -> Option<&str> {
        if self.token_ix <= self.tokens.len() {
            Some(self.tokens[self.token_ix])
        } else {
            None
        }
    }

    fn parseValue(&mut self) -> Result<JSONValue,ParseError> {
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

    fn parseObject(&mut self) -> Result<JSONValue,ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseArray(&mut self) -> Result<JSONValue,ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseTrue(&mut self) -> Result<JSONValue,ParseError> {
        Ok(JSONValue::True)
    }

    fn parseFalse(&mut self) -> Result<JSONValue,ParseError> {
        Ok(JSONValue::False)
    }

    fn parseNull(&mut self) -> Result<JSONValue,ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseNameValuePairs(&mut self) -> Result<JSONValue,ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseNameValuePair(&mut self) -> Result<JSONValue,ParseError> {
        Ok(JSONValue::Null)
    }

    fn parseName(&mut self) -> Result<JSONValue,ParseError> {
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
