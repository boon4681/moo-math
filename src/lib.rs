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

pub trait IIter: Iterator {
    fn prev(&mut self) -> Option<Self::Item>;
}

pub struct Iter<'a, Item> where Item: 'a {
    index: Option<usize>,
    vector: &'a Vec<Item>,
}

impl<'a, Item> Iter<'a, Item> {
    fn new(vector: &'a Vec<Item>) -> Iter<'a, Item> {
        Iter { index: None, vector }
    }
}

impl<'a, Item> Iterator for Iter<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<&'a Item> {
        let index =
            match self.index {
                Some(i) => i + 1,
                None => 0
            };

        self.index = Some(index);
        self.vector.get(index)
    }
}

impl<'a, Item> IIter for Iter<'a, Item> {
    fn prev(&mut self) -> Option<&'a Item> {
        let index =
            match self.index {
                Some(0) | None => return None,
                Some(i) => i - 1
            };

        self.index = Some(index);
        self.vector.get(index)
    }
}

#[derive(Debug)]
pub struct Program {
    pub body: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Primitive(Primitive),
    ExpressionOperation((Box<Expression>, char, Box<Expression>)),
}

#[derive(Debug)]
pub enum Primitive {
    Function((String, Box<Expression>)),
    Identifier(String),
    Parenthesis(Box<Expression>),
    Number(f64),
}

impl Primitive {
    fn typ(&self) -> &'static str {
        use Primitive::*;
        match self {
            Function(_) => "Function",
            Parenthesis(_) => "Parenthesis",
            Identifier(_) => "Identifier",
            Number(_) => "Number",
        }
    }
    fn perform(&self) -> f64 {
        use Primitive::*;
        match self {
            Function(func) => {
                return 0.0;
            }
            Parenthesis(expr) => {
                return 0.0;
            }
            Identifier(_) => {
                return 0.0;
            }
            Number(num) => *num
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
    Pow,
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
            Add | Sub | Mult | Div | Pow => "Operator",
            LParent => "LParent",
            RParent => "RParent",
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
    return match num_str.replace("_", "").parse::<f64>() {
        Ok(num) => Ok(Some((num, i))),
        Err(_) => Err("Cannot to parse number"),
    };
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
            try_one_char!(self, start, ch, '^', Pow);
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

pub struct Moo<'a> {
    source: &'a str,
    functions: HashMap<String, Function>,
}

impl<'a> Moo<'a> {
    fn new(source: &str) -> Moo {
        Moo {
            source,
            functions: Default::default(),
        }
    }
    fn parse(&self) -> Result<Option<Program>, &str> {
        let mut tokenizer = Tokenizer::new(self.source);
        let mut tokens: Vec<(Token, usize, usize)> = Vec::new();
        loop {
            let result = tokenizer.next();
            if result.is_ok() {
                match result.unwrap() {
                    Some(token) => tokens.push(token),
                    None => break,
                }
            } else {
                break;
            }
        }
        Ok(self.ast_program(&mut Iter::new(&tokens))?)
    }
    fn ast_program(&self, iter: &mut Iter<(Token, usize, usize)>) -> Result<Option<Program>, &str> {
        let body = self.ast_additive_expression(iter)?;
        if body.is_some() {
            return Ok(Some(Program { body: body.unwrap() }));
        }
        return Ok(None);
    }
    fn ast_additive_expression(&self, iter: &mut Iter<(Token, usize, usize)>) -> Result<Option<Expression>, &str> {
        let left = self.ast_multiplicative_expression(iter)?;
        if left.is_none() {
            return Ok(None);
        }
        if let Some(tnk) = iter.next() {
            use crate::Token::*;
            use crate::Expression::*;
            match tnk.0 {
                Add => {
                    let right = self.ast_additive_expression(iter)?;
                    if right.is_none() {
                        return Err("ERROR 8");
                    }
                    return Ok(Some(ExpressionOperation((Box::new(left.unwrap()), '+', Box::new(right.unwrap())))));
                }
                Sub => {
                    let right = self.ast_additive_expression(iter)?;
                    if right.is_none() {
                        return Err("ERROR 7");
                    }
                    return Ok(Some(ExpressionOperation((Box::new(left.unwrap()), '-', Box::new(right.unwrap())))));
                }
                _ => {
                    iter.prev();
                }
            };
        }
        Ok(left)
    }
    fn ast_multiplicative_expression(&self, iter: &mut Iter<(Token, usize, usize)>) -> Result<Option<Expression>, &str> {
        let left = self.ast_exponential_expression(iter)?;
        if left.is_none() {
            return Ok(None);
        }
        if let Some(tnk) = iter.next() {
            use crate::Token::*;
            use crate::Expression::*;
            match tnk.0 {
                Mult => {
                    let right = self.ast_exponential_expression(iter)?;
                    if right.is_none() {
                        return Err("ERROR 6");
                    }
                    return Ok(Some(ExpressionOperation((Box::new(left.unwrap()), '*', Box::new(right.unwrap())))));
                }
                Div => {
                    let right = self.ast_exponential_expression(iter)?;
                    if right.is_none() {
                        return Err("ERROR 5");
                    }
                    return Ok(Some(ExpressionOperation((Box::new(left.unwrap()), '/', Box::new(right.unwrap())))));
                }
                _ => {
                    iter.prev();
                }
            };
        }
        Ok(left)
    }

    fn ast_exponential_expression(&self, iter: &mut Iter<(Token, usize, usize)>) -> Result<Option<Expression>, &str> {
        use crate::Token::*;
        use crate::Expression::*;
        let left = self.ast_primitive(iter)?;
        if left.is_none() {
            return Ok(None);
        }
        if let Some(tnk) = iter.next() {
            match tnk.0 {
                Pow => {
                    let right = self.ast_primitive(iter)?;
                    if right.is_none() {
                        return Err("ERROR 4");
                    }
                    return Ok(Some(ExpressionOperation((Box::new(left.unwrap()), '^', Box::new(right.unwrap())))));
                }
                _ => {
                    iter.prev();
                }
            };
        }
        match left.unwrap() {
            Primitive(left) => Ok(Some(Primitive(left))),
            ExpressionOperation(left) => Ok(Some(ExpressionOperation(left)))
        }
    }

    fn ast_primitive(&self, iter: &mut Iter<(Token, usize, usize)>) -> Result<Option<Expression>, &str> {
        use crate::Token::*;
        return if let Some(tnk) = iter.next() {
            match &tnk.0 {
                Identifier(ident) => {
                    return match ident.as_str() {
                        "x" => {
                            Ok(Some(Expression::Primitive(Primitive::Identifier(ident.clone()))))
                        }
                        _ => Err("ERROR 3")
                    };
                    // return Ok(Expression::Primitive(Primitive::))
                }
                Number(num) => {
                    return Ok(Some(Expression::Primitive(Primitive::Number(num.clone()))));
                }
                LParent => {}
                RParent => {
                    return Err("ERROR 2");
                }
                _ => {
                    return Err("ERROR 1");
                }
            }
            Ok(None)
        } else {
            Ok(None)
        };
    }
}

#[cfg(test)]
mod numeric_tests {
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

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn parse_1() {
        let mut moo = Moo::new("10 - 20 ^ 5");
        println!("{:?}", moo.parse().ok());
        println!("{:?}", moo.parse().err());
    }
}
