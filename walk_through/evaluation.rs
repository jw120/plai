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
    List(Vec<SExp>),
}

type Symbol = String;

fn read_sexp(s: &str) -> Result<(SExp, &str)> {
    println!("read_sexp '{s}'");
    let s = s.trim_start();
    let mut chars = s.chars();
    let Some(first_char) = chars.next() else {
        bail!("Empty string in read");
    };
    if first_char.is_ascii_digit() {
        read_number(s)
    } else if first_char == '-' {
        if let Some(second_char) = chars.next() && second_char.is_ascii_digit() {
            read_number(s)
        } else {
            read_symbol(s)
        }
    } else if first_char == '#' {
        read_boolean(s)
    } else if first_char == '{' {
        read_list(s)
    } else if is_symbol_char(first_char) {
        read_symbol(s)
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
    match s.char_indices().find(|(_, c)| !is_symbol_char(*c)) {
        Some((0, _)) => Err(anyhow!("Internal error - expected symbol at '{s}'")),
        Some((r, _)) => Ok((SExp::Symbol(s[..r].to_string()), &s[r..])),
        None => Ok((SExp::Symbol(s.to_owned()), "")),
    }
}

fn read_list(s: &str) -> Result<(SExp, &str)> {
    println!("read_list '{s}'");
    let Some(without_brace) = s.strip_prefix('{') else {
        bail!("Internal error - no brace in read_list");
    };
    let mut items: Vec<SExp> = Vec::new();
    let mut remaining = without_brace;
    loop {
        println!("  loop '{remaining}' {items:?}");
        remaining = remaining.trim_start();
        if remaining.is_empty() {
            bail!("Unterminated list");
        }
        if let Some(rest) = remaining.strip_prefix('}') {
            return Ok((SExp::List(items), rest));
        }
        let (sexp, new_remaining) = read_sexp(remaining)?;
        remaining = new_remaining;
        items.push(sexp);
    }
}

fn is_not_numeric((i, c): &(usize, char)) -> bool {
    !(c.is_ascii_digit() || (*i == 0 && *c == '-'))
}

fn is_symbol_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '+' || c == '-' || c == '*' || c == '/'
}

// Expressions

#[derive(Debug, Eq, PartialEq)]
enum Exp {
    Number(i32),
    Boolean(bool),
    Variable(Symbol),
    Cond(Box<Exp>, Box<Exp>, Box<Exp>),
    //    Let1(Symbol, Box<Exp>, Box<Exp>),
    Lambda(Symbol, Box<Exp>),
    //    Apply(Box<Exp>, Box<Exp>),
    Add(Box<Exp>, Box<Exp>),
    Sub(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Div(Box<Exp>, Box<Exp>),
}

impl Exp {
    fn show_plait(&self) -> String {
        match self {
            Self::Number(i) => format!("(numE {i})"),
            Self::Boolean(true) => "(boolE #t)".to_owned(),
            Self::Boolean(false) => "(boolE #f)".to_owned(),
            Self::Variable(s) => format!("(varE '{s})"),
            Self::Cond(c, a, b) => format!(
                "(cndE {} {} {})",
                c.show_plait(),
                a.show_plait(),
                b.show_plait()
            ),
            Self::Lambda(s, b) => format!("(lamE '{s} {})", b.show_plait()),
            Self::Add(l, r) => format!("(addE {} {})", l.show_plait(), r.show_plait()),
            Self::Sub(l, r) => format!("(subE {} {})", l.show_plait(), r.show_plait()),
            Self::Mul(l, r) => format!("(mulE {} {})", l.show_plait(), r.show_plait()),
            Self::Div(l, r) => format!("(divE {} {})", l.show_plait(), r.show_plait()),
        }
    }
}

fn parse_sexp(sexp: &SExp) -> Result<Exp> {
    println!("parse_sexp {sexp:?}");
    match sexp {
        SExp::Number(n) => return Ok(Exp::Number(*n)),
        SExp::Boolean(b) => return Ok(Exp::Boolean(*b)),
        SExp::Symbol(s) => return Ok(Exp::Variable(s.clone())),
        SExp::List(v) => match &v[..] {
            [SExp::Symbol(f), a, b, c] => {
                if f == "if" {
                    return Ok(Exp::Cond(
                        Box::new(parse_sexp(a)?),
                        Box::new(parse_sexp(b)?),
                        Box::new(parse_sexp(c)?),
                    ));
                }
            }
            [SExp::Symbol(f), a, b] => match f.as_str() {
                "+" => return Ok(Exp::Add(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?))),
                "-" => return Ok(Exp::Sub(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?))),
                "*" => return Ok(Exp::Mul(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?))),
                "/" => return Ok(Exp::Div(Box::new(parse_sexp(a)?), Box::new(parse_sexp(b)?))),
                "lam" => if let SExp::Symbol(variable) = a {
                    return Ok(Exp::Lambda(variable.clone(), Box::new(parse_sexp(b)?)))
                } else {
                    return Err(anyhow!("unrecognized lam form in list"))
                }
                _ => return Err(anyhow!("unrecognized symbol in list")),
            },
            _ => {}
        },
    }
    Err(anyhow!("Unrecognized form in s-expression: {sexp:?}"))
}

/// Parse from string to an expression and convert to plait-style
pub fn parse(s: &str) -> Result<String> {
    let (sexp, rest) = read_sexp(s)?;
    if !rest.trim().is_empty() {
        bail!("Leftover string in parse");
    }
    let exp = parse_sexp(&sexp)?;
    Ok(exp.show_plait())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_boolean() {
        assert_eq!(read_boolean("#f").unwrap(), (SExp::Boolean(false), ""));
        assert_eq!(
            read_boolean("#false QQ").unwrap(),
            (SExp::Boolean(false), " QQ")
        );
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
    fn test_read_symbol() {
        assert_eq!(
            read_symbol("x").unwrap(),
            (SExp::Symbol("x".to_string()), "")
        );
        assert_eq!(
            read_symbol("AXE)").unwrap(),
            (SExp::Symbol("AXE".to_string()), ")")
        );
    }

    #[test]
    fn test_read_list() {
        assert_eq!(
            read_list("{a 2}").unwrap(),
            (
                SExp::List(vec![SExp::Symbol("a".to_string()), SExp::Number(2)]),
                ""
            )
        );
    }

    #[test]
    fn test_read_sexp() {
        assert_eq!(read_sexp("  234").unwrap(), (SExp::Number(234), ""));
        assert_eq!(read_sexp(" #true)").unwrap(), (SExp::Boolean(true), ")"));
        assert_eq!(
            read_sexp(" z)").unwrap(),
            (SExp::Symbol("z".to_string()), ")")
        );
        assert_eq!(read_sexp("-").unwrap(), (SExp::Symbol("-".to_string()), ""));
        assert_eq!(read_sexp("-2").unwrap(), (SExp::Number(-2), ""));
    }

    #[test]
    fn test_parse_binary_mul() {
        let (sexp, rest) = read_sexp("{* 2 3}").unwrap();
        assert!(rest.is_empty());
        assert_eq!(sexp, SExp::List(vec![SExp::Symbol("*".to_string()), SExp::Number(2), SExp::Number(3)]));
        let exp = parse_sexp(&sexp).unwrap();
        assert_eq!(exp, Exp::Mul(Box::new(Exp::Number(2)), Box::new(Exp::Number(3))));
    }

    #[test]
    fn test_parse_binary_add() {
        let (sexp, rest) = read_sexp("{+ 22 33}").unwrap();
        assert!(rest.is_empty());
        assert_eq!(sexp, SExp::List(vec![SExp::Symbol("+".to_string()), SExp::Number(22), SExp::Number(33)]));
        let exp = parse_sexp(&sexp).unwrap();
        assert_eq!(exp, Exp::Add(Box::new(Exp::Number(22)), Box::new(Exp::Number(33))));
    }

    #[test]
    fn test_parse_if() {
        let (sexp, rest) = read_sexp("{if x 2 {+ y 4}}").unwrap();
        assert!(rest.is_empty());
        assert_eq!(sexp, SExp::List(vec![
            SExp::Symbol("if".to_string()),
            SExp::Symbol("x".to_string()),
            SExp::Number(2),
            SExp::List(vec![SExp::Symbol("+".to_string()), SExp::Symbol("y".to_string()), SExp::Number(4)])]));
        let exp = parse_sexp(&sexp).unwrap();
        assert_eq!(exp, Exp::Cond(
            Box::new(Exp::Variable("x".to_string())),
            Box::new(Exp::Number(2)),
            Box::new(Exp::Add(Box::new(Exp::Variable("y".to_string())), Box::new(Exp::Number(4))))));
    }
}
