#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ident,
    Number,
    LParen,
    RParen,
}

pub fn whitespace(mut input: &str) -> &str {
    while matches!(peek_char(input), Some(' ')) {
        let mut chars = input.chars();
        chars.next();
        input = chars.as_str();
    }
    input
}

pub fn number(mut input: &str) -> (&str, Option<Token>) {
    if matches!(
        peek_char(input),
        Some(_x @ ('-' | '+' | '.' | '0'..='9'))
    ) {
        while matches!(
            peek_char(input),
            Some(_x @ ('-' | '+' | '.' | '0'..='9'))
        ) {
            input = advance_char(input);
        }
        return (input, Some(Token::Number));
    }
    return (input, None);
}

pub fn ident(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(_x @ ('a'..='z' | 'A'..='Z'))) {
        while matches!(
            peek_char(input),
            Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9'))
        ) {
            input = advance_char(input);
        }
        return (input, Some(Token::Ident));
    }
    return (input, None);
}

pub fn lparen(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some('(')) {
        return (advance_char(input), Some(Token::LParen));
    }
    (input, None)    
}

pub fn rparen(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(')')) {
        return (advance_char(input), Some(Token::RParen));
    }
    (input, None)    
}

pub fn token(i: &str) -> (&str, Option<Token>) {
    if let (i, Some(ident_res)) = ident(whitespace(i)) {
        return (i, Some(ident_res));
    }

    if let (i, Some(number_res)) = number(whitespace(i)) {
        return (i, Some(number_res));
    }

    if let(i, Some(lparen_res)) = lparen(whitespace(i)) {
        return (i, Some(lparen_res))
    }

    if let(i, Some(rparen_res)) = rparen(whitespace(i)) {
        return (i, Some(rparen_res))
    }
 
    (i, None)
}

fn tokenize(mut input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    while !input.is_empty() {
        input = if let (next_input, Some(token)) = token(input) {
            tokens.push(token);
            next_input
        } else {
            break;
        }
    }
    tokens
}

fn advance_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

fn peek_char(input: &str) -> Option<char> {
    input.chars().next()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespace() {
        let input = "  ";
        let res = whitespace(input);
        assert_eq!("", res);
    }

    #[test]
    fn test_number() {
        let input = "123.45";
        let (rest, token) = number(input);
        assert_eq!("", rest);
        assert_eq!(token, Some(Token::Number));
    }

    #[test]
    fn test_ident() {
        let input = "test1";
        let (rest, token) = ident(input);
        assert_eq!("", rest);
        assert_eq!(token, Some(Token::Ident));
    }

    #[test]
    fn test_tokenize() {
        let input = "(test 12 test1  100.00)";
        let tokens = tokenize(input);

        let expected = vec![
            Token::LParen,
            Token::Ident,
            Token::Number,
            Token::Ident,
            Token::Number,
            Token::RParen,
        ];

        assert_eq!(tokens, expected);
    }
}
