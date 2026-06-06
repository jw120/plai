use anyhow::{anyhow, bail, Result};

type Symbol = String;

#[derive(Debug, Eq, PartialEq)]
enum Exp {
    Number(i32),
    Boolean(bool),
    Variable(Symbol),
    Cond(Box<Exp>, Box<Exp>, Box<Exp>),
    Let1(Symbol, Box<Exp>, Box<Exp>),
    Lambda(Symbol, Box<Exp>),
    Apply(Box<Exp>, Box<Exp>),
    Add(Box<Exp>, Box<Exp>),
    Sub(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Div(Box<Exp>, Box<Exp>),
}

fn parse(s: &str) -> Result<(Exp, &str)> {
    let s = s.trim_start();
    let Some(first_char) = s.chars().next() else {
        bail!("Empty string in parse");
    };
    if first_char.is_ascii_digit() || first_char == '-' {
        parse_number(s)
    } else if first_char == '#' {
        parse_boolean(s)
    } else if first_char.is_ascii_alphabetic() {
        parse_variable(s)
    } else {
        Err(anyhow!("Unrecognized string to parse: '{s}'"))
    }
}

fn parse_boolean(s: &str) -> Result<(Exp, &str)> {
    if let Some(rest) = s.strip_prefix("#true") {
        return Ok((Exp::Boolean(true), rest));
    }
    if let Some(rest) = s.strip_prefix("#t") {
        return Ok((Exp::Boolean(true), rest));
    }
    if let Some(rest) = s.strip_prefix("#false") {
        return Ok((Exp::Boolean(false), rest));
    }
    if let Some(rest) = s.strip_prefix("#f") {
        return Ok((Exp::Boolean(false), rest));
    }
    Err(anyhow!("Internal error - expected boolean at '{s}'"))
}

fn parse_number(s: &str) -> Result<(Exp, &str)> {
    match s.char_indices().find(is_not_numeric) {
        Some((0, _)) => Err(anyhow!("Internal error - expected number at '{s}'")),
        Some((r, _)) => Ok((Exp::Number(s[..r].parse()?), &s[r..])),
        None => Ok((Exp::Number(s.parse()?), "")),
    }
}

fn parse_variable(s: &str) -> Result<(Exp, &str)> {
    match s.char_indices().find(|(_, c)| !c.is_ascii_alphabetic()) {
        Some((0, _)) => Err(anyhow!("Internal error - expected symbol at '{s}'")),
        Some((r, _)) => Ok((Exp::Variable(s[..r].to_string()), &s[r..])),
        None => Ok((Exp::Variable(s.to_string()), "")),
    }
   
}

fn is_not_numeric((i, c) : &(usize, char)) -> bool {
    !(c.is_ascii_digit() || (*i == 0 && *c == '-'))
}





fn main() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_boolean() {
        assert_eq!(parse_boolean("#f").unwrap(), (Exp::Boolean(false), ""));
        assert_eq!(parse_boolean("#false QQ").unwrap(), (Exp::Boolean(false), " QQ"));
        assert_eq!(parse_boolean("#t").unwrap(), (Exp::Boolean(true), ""));
        assert_eq!(parse_boolean("#true)").unwrap(), (Exp::Boolean(true), ")"));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("23").unwrap(), (Exp::Number(23), ""));
        assert_eq!(parse_number("42)").unwrap(), (Exp::Number(42), ")"));
        assert_eq!(parse_number("-1 )").unwrap(), (Exp::Number(-1), " )"));
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(parse_variable("x").unwrap(), (Exp::Variable("x".to_string()), ""));
        assert_eq!(parse_variable("AXE)").unwrap(), (Exp::Variable("AXE".to_string()), ")"));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("  234").unwrap(), (Exp::Number(234), ""));
        assert_eq!(parse(" #true)").unwrap(), (Exp::Boolean(true),")"));
        assert_eq!(parse(" z)").unwrap(), (Exp::Variable("z".to_string()),")"));
    }
}
