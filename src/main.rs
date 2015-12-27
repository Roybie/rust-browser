use std::iter::Peekable;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Token {
    LANGLE,
    RANGLE,
    LANGLESLASH,
    NUMBER(f64),
    WORD(String),
}

pub struct Tokeniser<I: Iterator> {
    iter: Peekable<I>
}

impl<I: Iterator<Item=char>> Tokeniser<I> {
    pub fn new(input: I) -> Tokeniser<I> {
        Tokeniser {
            iter: input.peekable(),
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for Tokeniser<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match *self.iter.peek().unwrap_or(&'⊥') {
            '<'                     => {
                self.iter.next();
                if self.iter.peek().unwrap_or(&'⊥').eq(&'/') {
                    self.iter.next();
                    Some(Token::LANGLESLASH)
                } else {
                    Some(Token::LANGLE)
                }
            },
            '>'                     => { self.iter.next(); Some(Token::RANGLE) },
            'a'...'z'|'A'...'Z'|'0'...'9'|'.'|'-'     => {
                let mut s = String::new();
                while self.iter.peek().unwrap_or(&'⊥').is_alphanumeric() || ".-".chars().any(|c| c == *self.iter.peek().unwrap_or(&'⊥')) {
                    if let Some(stri) = self.iter.next() {
                        s = s + &stri.to_string();
                    }
                }
                match s.parse::<f64>() {
                    Ok(d)  => Some(Token::NUMBER(d)),
                    _      => Some(Token::WORD(s.to_owned())),
                }
            },
            '⊥'                     => None,
            _                       => { self.iter.next(); self.next() }
        }
    }
}


fn main() {
    let input = "<html><head> </head><body>Some body\n-5.98</body></html>".chars();

    let tokeniser = Tokeniser::new(input);

    for token in tokeniser {
        println!("{:?}", token);
    }
}
