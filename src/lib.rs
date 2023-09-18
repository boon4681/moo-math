// order of operations
// parentheses
// exponentiation
// multiplication and division
// addition and subtraction
// number | function

use std::collections::HashMap;

macro_rules! cast_opt_ok {
    ($i:expr) => {
        match $i {
            Some(e) => e,
            None => return Ok(None),
        }
    };
}

macro_rules! try_tokenize {
    ($self:ident,$start:ident,$i:ident,$j:ident) => {{
        let result = $i($self.source)?;
        if result.is_some() {
            let option = result.unwrap();
            $self.shift(option.1);
            return Ok(Some((Token::$j(option.0), $start, $self.index)));
        }
    }};
}

macro_rules! try_one_char {
    ($self:ident,$start:ident,$ch:ident,$i:expr,$j:ident) => {{
        if $ch == $i {
            $self.shift(1);
            return Ok(Some((Token::$j, $self.index - 1, $self.index)));
        }
    }};
}

pub struct Function {
    pub name: String,
    pub func: fn(f64) -> f64,
}

impl Function {
    fn new(name: &str, func: fn(f64) -> f64) -> Self {
        Self {
            name: name.to_string(),
            func,
        }
    }
}

pub enum Token {
    Identifier(String),
    Number(f64),
    Add,
    Sub,
    Mult,
    Div,
    LParent,
    RParent,
    Comma,
}

impl Token {
    fn typ(&self) -> &'static str {
        use Token::*;
        match self {
            Identifier(_) => "Identifier",
            Number(_) => "Number",
            Add | Sub | Mult | Div => "Operator",
            LParent | RParent => "Parenthesis",
            Comma => "Comma",
        }
    }
}

struct Tokenizer<'a> {
    index: usize,
    source: &'a str,
}

fn takes(source: &str, f: fn(ch: char) -> bool) -> Result<(&str, usize), &str> {
    let mut i: usize = 0;
    for ch in source.chars() {
        if f(ch) {
            i += 1;
        } else {
            break;
        }
    }
    if i == 0 {
        Err("No Matches".into())
    } else {
        Ok((&source[..i], i))
    }
}

fn identifier(source: &str) -> Result<Option<(String, usize)>, &str> {
    let mut i: usize = 0;
    let mut chars = source.chars();
    let ch = cast_opt_ok!(chars.next());
    if !(ch.is_alphabetic() || ch == '_') {
        return Ok(None);
    }
    i += 1;
    for ch in chars {
        if ch.is_alphanumeric() || ch == '_' {
            i += 1;
        } else {
            break;
        }
    }
    if i > 0 {
        Ok(Some((source[..i].to_string(), i)))
    } else {
        Ok(None)
    }
}

fn number(source: &str) -> Result<Option<(f64, usize)>, &str> {
    let mut i: usize = 0;
    let result = integer(source)?;
    if result.is_none() {
        return Ok(None);
    }
    let int = result.unwrap();
    i += int.1;
    let num_str;
    if i < source.len() {
        let mut chars = source.get(i..).unwrap().chars();
        let ch = chars.next().unwrap();
        if ch == '.' {
            for ch in chars {
                if ch == '.' {
                    return Err("Unexpected number");
                }
                if ch.is_digit(10) {
                    i += 1;
                }
            }
            num_str = &source[..i];
        } else {
            num_str = int.0;
        }
    } else {
        num_str = int.0;
    }
    match num_str.replace("_", "").parse::<f64>() {
        Ok(num) => return Ok(Some((num, i))),
        Err(_) => return Err("Cannot to parse number"),
    }
}

fn integer(source: &str) -> Result<Option<(&str, usize)>, &str> {
    let mut i: usize = 0;
    let mut chars = source.chars();
    let ch = cast_opt_ok!(chars.next());
    if ch == '0' {
        return Ok(Some(("0", 1)));
    }
    if ch.is_digit(10) {
        i += 1;
        for ch in chars {
            if ch.is_digit(10) || ch == '_' {
                i += 1;
            } else {
                break;
            }
        }
        return Ok(Some((&source[..i], i)));
    }
    return Ok(None);
}

impl<'a> Tokenizer<'a> {
    fn new(src: &str) -> Tokenizer {
        Tokenizer {
            index: 0,
            source: src,
        }
    }
    fn next(&mut self) -> Result<Option<(Token, usize, usize)>, &str> {
        if !self.source.is_empty() {
            self.skip_whitespace();
            let mut ch = self.source.chars().next().unwrap();
            let start = self.index;
            try_tokenize!(self, start, number, Number);
            try_tokenize!(self, start, identifier, Identifier);
            try_one_char!(self, start, ch, '+', Add);
            try_one_char!(self, start, ch, '-', Sub);
            try_one_char!(self, start, ch, '*', Mult);
            try_one_char!(self, start, ch, '/', Div);
            try_one_char!(self, start, ch, '(', LParent);
            try_one_char!(self, start, ch, ')', RParent);
            try_one_char!(self, start, ch, ',', Comma);
            Ok(None)
        } else {
            Ok(None)
        }
    }
    fn skip_whitespace(&mut self) {
        match takes(self.source, |a| a.is_whitespace()) {
            Ok(e) => {
                self.shift(e.1);
            }
            Err(_) => {}
        };
    }
    fn shift(&mut self, length: usize) {
        self.source = &self.source[length..];
        self.index += length;
    }
}

pub struct Moo {
    functions: HashMap<String, Function>,
}

impl Moo {}

#[cfg(test)]
mod numuric_tests {
    use super::*;
    #[test]
    fn number_integer() {
        let mut tokenizer = Tokenizer::new(" 10");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Number");
    }
    #[test]
    fn number_zero_integer() {
        let mut tokenizer = Tokenizer::new(" 0");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Number");
    }
    #[test]
    fn number_float() {
        let mut tokenizer = Tokenizer::new(" 10.1");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Number");
    }
    #[test]
    fn number_zero_float() {
        let mut tokenizer = Tokenizer::new(" 0.1");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Number");
    }
    #[test]
    fn number_underscore() {
        let mut tokenizer = Tokenizer::new(" 1_10");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Number");
    }
    #[test]
    fn number_unexpected_err() {
        let mut tokenizer = Tokenizer::new(" 10.1.1");
        assert_eq!(tokenizer.next().err().unwrap(), "Unexpected number");
    }
}

#[cfg(test)]
mod identifier_tests {
    use super::*;
    #[test]
    fn ident_x() {
        let mut tokenizer = Tokenizer::new("x");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Identifier");
    }
    #[test]
    fn ident_cow() {
        let mut tokenizer = Tokenizer::new("วัว");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Identifier");
    }
    #[test]
    fn ident_underscore() {
        let mut tokenizer = Tokenizer::new("_cow");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Identifier");
    }
    #[test]
    fn ident_number() {
        let mut tokenizer = Tokenizer::new("boon1_");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Identifier");
    }
}

#[cfg(test)]
mod expr_tests {
    use super::*;
    #[test]
    fn expr_1() {
        let mut tokenizer = Tokenizer::new(" 10  + x");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Number");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Operator");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Identifier");
    }
    #[test]
    fn expr_2() {
        let mut tokenizer = Tokenizer::new(" (10 / 5)");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Parenthesis");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Number");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Operator");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Number");
        assert_eq!(tokenizer.next().unwrap().unwrap().0.typ(), "Parenthesis");
    }
}
