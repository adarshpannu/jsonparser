// JSON Parser (as if the world needs another one)

#![allow(warnings)]

pub struct JSONParser<'a> {
    tokens: Vec<&'a str>
}

impl<'a> JSONParser<'a> {
    fn parse(s: &str) -> JSONParser {
        let tokens = Self::tokenize(s);
        JSONParser { tokens }
    }

    fn tokenize(s: &str) -> Vec<&str> {
        s.split(|c| c == ' ' || c == '\n')
            .filter(|&s| s.len() > 0)
            .collect::<Vec<&str>>()
    }
}

enum Token {
    OpenBrace,
    CloseBrace,
    Identifier,
    StringLiteral,
    NumericLiteral,
    Comma
}

// State machine
// State + Token -> function?

// Stack

impl<'a> JSONParser<'a> {
    fn parseValue() {

    }

    fn parseObject() {

    }

    fn parseNameValuePairs() {

    }

    fn parseNameValuePair() {

    }

    fn parseName() {

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

    dbg!(JSONParser::tokenize(jsonstr));
}
