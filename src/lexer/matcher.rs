use lexer::tokenizer::Tokenizer;
use lexer::token::{Token, TokenType};

pub trait Matcher {
    fn try_match(&self, tok: &mut Tokenizer) -> Option<Token>;
}

#[allow(dead_code)]
pub struct MatchWhitespace {

}

impl Matcher for MatchWhitespace {
    fn try_match(&self, tok: &mut Tokenizer) -> Option<Token> {
        let mut found_ws = false;

        while !tok.end() && " \n\r".contains(tok.peek().unwrap()) {
            found_ws = true;
            tok.next();
        }

        if found_ws {
            return Some(Token { tok_type: TokenType::WhiteSpace });
        }

        None
    }
}

#[allow(dead_code)]
pub struct MatchNumber {

}

impl Matcher for MatchNumber {
    fn try_match(&self, tok: &mut Tokenizer) -> Option<Token> {
        let mut number = String::new();

        while !tok.end() && "0123456789".contains(tok.peek().unwrap()) {
            number.push(tok.next().unwrap());
        }

        if !number.is_empty() {
            return Some(Token { tok_type: TokenType::IntLiteral(number.parse::<u64>().unwrap()) });
        }

        None
    }
}

#[allow(dead_code)]
pub struct MatchSymbol {
    symbols: Vec<String>,
}

impl MatchSymbol {
    #[allow(dead_code)]
    pub fn new(symbols: Vec<String>) -> MatchSymbol {
        MatchSymbol { symbols: symbols }
    }
}

impl Matcher for MatchSymbol {
    fn try_match(&self, tok: &mut Tokenizer) -> Option<Token> {
        for symbol in self.symbols.clone() {
            let dat = tok.clone().take(symbol.len());
            if dat.size_hint().1.unwrap() != symbol.len() {
                return None;
            }

            if dat.collect::<String>() == symbol {
                tok.advance(symbol.len());
                return Some(Token { tok_type: TokenType::Symbol(symbol) });
            }
        }
        None
    }
}

#[allow(dead_code)]
pub struct MatchIdentifier {

}

impl Matcher for MatchIdentifier {
    fn try_match(&self, tok: &mut Tokenizer) -> Option<Token> {
        let mut identifier = String::new();

        let curr = tok.next().unwrap();
        if curr == '_' || curr.is_alphabetic() {
            identifier.push(curr);
        } else {
            return None;
        }

        loop {
            if tok.end() {
                break;
            }
            let current = tok.peek().unwrap();
            if !current.is_whitespace() && (current == '_' || current.is_alphanumeric()) {
                identifier.push(tok.next().unwrap());
            } else {
                break;
            }
        }

        if !identifier.is_empty() {
            return Some(Token { tok_type: TokenType::Identifier(identifier) });
        }

        None
    }
}
