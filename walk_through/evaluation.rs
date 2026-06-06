use anyhow::{anyhow, bail, Result};

// S-expressions
//
// We read into an s-expression before parsing to an expression, mimicking the
// plait code

#[derive(Debug, Eq, PartialEq)]
enum SExp {
    Number(i32),
    Boolean(bool),
    Symbol(Symbol),
    List(Vec<SExp>)
}

type Symbol = String;

fn read_sexp(s: &str) -> Result<(SExp, &str)> {
    println!("read_sexp {s}");
    let s = s.trim_start();
    let Some(first_char) = s.chars().next() else {
        bail!("Empty string in read");
    };
    if first_char.is_ascii_digit() || first_char == '-' {
        read_number(s)
    } else if first_char == '#' {
        read_boolean(s)
    } else if first_char.is_ascii_alphabetic() {
        read_symbol(s)
    } else if first_char == '{' {
        read_list(s)
    } else {
        Err(anyhow!("Unrecognized string to read: '{s}'"))
    }
}

fn read_boolean(s: &str) -> Result<(SExp, &str)> {
    if let Some(rest) = s.strip_prefix("#true") {
        return Ok((SExp::Boolean(true), rest));
    }
    if let Some(rest) = s.strip_prefix("#t") {
        return Ok((SExp::Boolean(true), rest));
    }
    if let Some(rest) = s.strip_prefix("#false") {
        return Ok((SExp::Boolean(false), rest));
    }
    if let Some(rest) = s.strip_prefix("#f") {
        return Ok((SExp::Boolean(false), rest));
    }
    Err(anyhow!("Internal error - expected boolean at '{s}'"))
}

fn read_number(s: &str) -> Result<(SExp, &str)> {
    match s.char_indices().find(is_not_numeric) {
        Some((0, _)) => Err(anyhow!("Internal error - expected number at '{s}'")),
        Some((r, _)) => Ok((SExp::Number(s[..r].parse()?), &s[r..])),
        None => Ok((SExp::Number(s.parse()?), "")),
    }
}

fn read_symbol(s: &str) -> Result<(SExp, &str)> {
    match s.char_indices().find(|(_, c)| !c.is_ascii_alphabetic()) {
        Some((0, _)) => Err(anyhow!("Internal error - expected symbol at '{s}'")),
        Some((r, _)) => Ok((SExp::Symbol(s[..r].to_string()), &s[r..])),
        None => Ok((SExp::Symbol(s.to_owned()), "")),
    }
   
}

fn read_list(s: &str) -> Result<(SExp, &str)> {
    println!("read_list {s}");
    let Some(without_brace) = s.strip_prefix('{') else {
        bail!("Internal error - no brace in read_list");
    };
    let mut items: Vec<SExp> = Vec::new();
    let mut remaining = without_brace;
    loop {
        println!("loop '{remaining}' {items:?}");
        remaining = remaining.trim_start();
        if remaining.is_empty() {
            bail!("Unterminated list");
        }
        if let Some(rest) = remaining.strip_prefix('}') {
            return Ok((SExp::List(items), rest))
        }            
        let (sexp, new_remaining) = read_sexp(remaining)?;
        remaining = new_remaining;
        items.push(sexp);        
    }
}

fn is_not_numeric((i, c) : &(usize, char)) -> bool {
    !(c.is_ascii_digit() || (*i == 0 && *c == '-'))
}

// Expressions

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

fn parse_sexp(sexp: &SExp) -> Result<Exp> {
     match sexp {
         SExp::Number(n) => return Ok(Exp::Number(*n)),
         SExp::Boolean(b) => return Ok(Exp::Boolean(*b)),
         SExp::Symbol(s) => return Ok(Exp::Variable(s.clone())),
         SExp::List(v) => match &v[..] {
             [SExp::Symbol(f), a, b, c] => if f == "if" {
                 return Ok(Exp::Cond(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?), Box::new(parse_sexp(c)?)))
             }
             [SExp::Symbol(f), SExp::Symbol(v), b] => if f == "lam" {
                 return Ok(Exp::Lambda(v.clone(), Box::new(parse_sexp(b)?)))
             }
             [SExp::Symbol(f), a, b] => match f.as_str() {
                 "+" => return Ok(Exp::Add(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?))),
                 "-" => return Ok(Exp::Sub(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?))),
                 "*" => return Ok(Exp::Mul(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?))),
                 "/" => return Ok(Exp::Div(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?))),
                 _ => return Err(anyhow!("unrecognized symbol in binary function")),
             }
             _ => {},
         }
     }
    Err(anyhow!("Unrecognized form in s-expression"))
}


fn main() {
    println!("{:?}", read_list("{a 2}"));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_boolean() {
        assert_eq!(read_boolean("#f").unwrap(), (SExp::Boolean(false), ""));
        assert_eq!(read_boolean("#false QQ").unwrap(), (SExp::Boolean(false), " QQ"));
        assert_eq!(read_boolean("#t").unwrap(), (SExp::Boolean(true), ""));
        assert_eq!(read_boolean("#true)").unwrap(), (SExp::Boolean(true), ")"));
    }

    #[test]
    fn test_read_number() {
        assert_eq!(read_number("23").unwrap(), (SExp::Number(23), ""));
        assert_eq!(read_number("42)").unwrap(), (SExp::Number(42), ")"));
        assert_eq!(read_number("-1 )").unwrap(), (SExp::Number(-1), " )"));
    }

    #[test]
    fn test_read_variable() {
        assert_eq!(read_symbol("x").unwrap(), (SExp::Symbol("x".to_string()), ""));
        assert_eq!(read_symbol("AXE)").unwrap(), (SExp::Symbol("AXE".to_string()), ")"));
    }

    #[test]
    fn test_read_list() {
        assert_eq!(read_list("{a 2}").unwrap(), (SExp::List(vec![SExp::Symbol("a".to_string()), SExp::Number(2)]), ""));
    }

    #[test]
    fn test_read_sexp() {
        assert_eq!(read_sexp("  234").unwrap(), (SExp::Number(234), ""));
        assert_eq!(read_sexp(" #true)").unwrap(), (SExp::Boolean(true),")"));
        assert_eq!(read_sexp(" z)").unwrap(), (SExp::Symbol("z".to_string()),")"));
    }
}
