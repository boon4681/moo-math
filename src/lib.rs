// order of operations
// parentheses
// exponentiation
// multiplication and division
// addition and subtraction
// number | function

mod utils;

use crate::utils::{IIter, Iter};
use std::collections::HashMap;
use std::f64;

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

#[derive(Clone, Debug)]
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
    Function((Function, Box<Expression>)),
    Identifier(String),
    Parenthesis(Box<Expression>),
    Number(f64),
}

impl Program {
    fn run(&self, x: f64) -> f64 {
        self.body.perform(x, 0.0)
    }
    fn runge_kutta(&self, x0: f64, y0: f64, step: f64) -> (f64, f64) {
        let a1 = step * self.body.perform(x0, y0);
        let a2 = step * self.body.perform(x0 + step / 2.0, y0 + a1 / 2.0);
        let a3 = step * self.body.perform(x0 + step / 2.0, y0 + a2 / 2.0);
        let a4 = step * self.body.perform(x0, y0 + a3);
        (y0 + (a1 + 2.0 * a2 + 2.0 * a3 + a4) / 6.0, x0 + step)
    }
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
    fn perform(&self, x: f64, y: f64) -> f64 {
        use Primitive::*;
        match self {
            Function(func) => {
                (func.0.func)(func.1.perform(x, y))
            }
            Parenthesis(expr) => {
                expr.perform(x, y)
            }
            Identifier(ident) => {
                match ident.as_str() {
                    "x" => x,
                    "y" => y,
                    _ => 0.0
                }
            }
            Number(num) => *num
        }
    }
}

impl Expression {
    fn perform(&self, x: f64, y: f64) -> f64 {
        use Expression::*;
        match self {
            Primitive(primitive) => {
                primitive.perform(x, y)
            }
            ExpressionOperation(expr) => {
                match expr.1 {
                    '+' => expr.0.perform(x, y) + expr.2.perform(x, y),
                    '-' => expr.0.perform(x, y) - expr.2.perform(x, y),
                    '*' => expr.0.perform(x, y) * expr.2.perform(x, y),
                    '/' => expr.0.perform(x, y) / expr.2.perform(x, y),
                    '^' => f64::powf(expr.0.perform(x, y), expr.2.perform(x, y)),
                    _ => 0.0
                }
            }
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
            if let Some(ch) = self.source.chars().next() {
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
            }
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
    functions: HashMap<&'a str, Function>,
}

impl<'a> Moo<'a> {
    fn new(add_on: fn(functions: &mut HashMap<&str, Function>)) -> Moo<'a> {
        let mut functions: HashMap<&str, Function> = HashMap::new();
        functions.insert("sin", Function::new("sin", |v| {
            f64::sin(v)
        }));
        functions.insert("cos", Function::new("cos", |v| {
            f64::sin(v)
        }));
        functions.insert("abs", Function::new("abs", |v| {
            f64::abs(v)
        }));
        add_on(&mut functions);
        Moo {
            functions,
        }
    }
    fn parse(&self, source: &str) -> Result<Option<Program>, &str> {
        let mut tokenizer = Tokenizer::new(source);
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
                    if ident.as_str() == "x" {
                        return Ok(Some(Expression::Primitive(Primitive::Identifier(ident.clone()))));
                    }
                    return match self.functions.get(&ident.as_str()) {
                        None => Err("ERROR 3"),
                        Some(func) => {
                            return match iter.next() {
                                Some(token) => {
                                    match token.0 {
                                        LParent => {}
                                        _ => {
                                            return Err("ERROR FUNCTION MUST HAVE OPEN BRACKET");
                                        }
                                    }
                                    let w_input = self.ast_additive_expression(iter)?;
                                    match w_input {
                                        None => Err("Function must have input"),
                                        Some(input) => {
                                            match iter.next() {
                                                Some(token) => {
                                                    match token.0 {
                                                        RParent => {}
                                                        _ => return Err("Function expected ')'")
                                                    }
                                                }
                                                None => return Err("Function expected ')'")
                                            }
                                            Ok(Some(Expression::Primitive(Primitive::Function((func.clone(), Box::new(input))))))
                                        }
                                    }
                                }
                                None => Err("Function expected '('"),
                            };
                        }
                    };
                }
                Number(num) => {
                    return Ok(Some(Expression::Primitive(Primitive::Number(num.clone()))));
                }
                LParent => {
                    let expr = self.ast_additive_expression(iter)?;
                    return match expr {
                        Some(expr) => {
                            match iter.next() {
                                Some(token) => {
                                    match token.0 {
                                        RParent => {}
                                        _ => return Err("Parenthesis expected ')'")
                                    }
                                }
                                None => return Err("Parenthesis expected ')'")
                            }
                            Ok(Some(Expression::Primitive(Primitive::Parenthesis(Box::new(expr)))))
                        }
                        None => {
                            match iter.next() {
                                Some(token) => {
                                    match token.0 {
                                        RParent => Ok(None),
                                        _ => return Err("Parenthesis expected ')'")
                                    }
                                }
                                None => return Err("Parenthesis expected ')'")
                            }
                        }
                    };
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
    use crate::Function;
    use super::*;

    #[test]
    fn parse_1() {
        let mut moo = Moo::new(|functions| {
            functions.insert("relu", Function::new("relu", |v| {
                f64::max(0.0, v)
            }));
        });
        println!("{:?}", moo.parse("abs(x)").ok().unwrap().unwrap().run(0.0));
        let program = moo.parse("relu(x)").ok().unwrap().unwrap();
        program.run(0.0);
    }
}
