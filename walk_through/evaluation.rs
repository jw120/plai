#![allow(dead_code)] // Code only use for testing

use std::fmt;

use anyhow::{Result, anyhow, bail};
use imbl::HashMap;

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

/// Read any s-expression, stripping leading spaces
fn read_sexp(s: &str) -> Result<(SExp, &str)> {
    // println!("read_sexp '{s}'");
    let s = s.trim_start();
    let mut chars = s.chars();
    let Some(first_char) = chars.next() else {
        bail!("Empty string in read");
    };
    if first_char.is_ascii_digit() {
        read_number(s)
    } else if first_char == '-' {
        if let Some(second_char) = chars.next()
            && second_char.is_ascii_digit()
        {
            read_number(s)
        } else {
            read_symbol(s)
        }
    } else if first_char == '#' {
        read_boolean(s)
    } else if first_char == '{' {
        read_list(s)
    } else if is_symbol_char(&(0, first_char)) {
        read_symbol(s)
    } else {
        Err(anyhow!("Unrecognized string to read: '{s}'"))
    }
}

/// Read a boolean (leading space causes an error0
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

/// Read a number (leading space causes an error0
fn read_number(s: &str) -> Result<(SExp, &str)> {
    match s.char_indices().find(|x| !is_numeric_char(x)) {
        Some((0, _)) => Err(anyhow!("Internal error - expected number at '{s}'")),
        Some((r, _)) => Ok((SExp::Number(s[..r].parse()?), &s[r..])),
        None => Ok((SExp::Number(s.parse()?), "")),
    }
}

/// Read a symbol (leading space causes an error0
fn read_symbol(s: &str) -> Result<(SExp, &str)> {
    match s.char_indices().find(|x| !is_symbol_char(x)) {
        Some((0, _)) => Err(anyhow!("Internal error - expected symbol at '{s}'")),
        Some((r, _)) => Ok((SExp::Symbol(s[..r].to_string()), &s[r..])),
        None => Ok((SExp::Symbol(s.to_owned()), "")),
    }
}

/// Read a list (leading space causes an error0
fn read_list(s: &str) -> Result<(SExp, &str)> {
    // println!("read_list '{s}'");
    let Some(without_brace) = s.strip_prefix('{') else {
        bail!("Internal error - no brace in read_list");
    };
    let mut items: Vec<SExp> = Vec::new();
    let mut remaining = without_brace;
    loop {
        //println!("  loop '{remaining}' {items:?}");
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

/// Helper function to check if character with index is part of a number
fn is_numeric_char((i, c): &(usize, char)) -> bool {
    c.is_ascii_digit() || (*i == 0 && *c == '-')
}

/// Helper function to check if character is part of a symbol
fn is_symbol_char((i, c): &(usize, char)) -> bool {
    c.is_ascii_alphabetic()
        || ['+', '-', '*', '/'].contains(c)
        || *c == '/'
        || (*i > 0 && c.is_ascii_digit())
    // c.is_ascii_alphabetic() || *c == '+' || *c == '-' || *c == '*' || *c == '/' || (*i > 0 && c.is_ascii_digit())
}

/// Binary operations for use in expressions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    fn apply(self, left: &Value, right: &Value) -> Result<Value> {
        if let Value::Number(l) = left {
            if let Value::Number(r) = right {
                match self {
                    Self::Add => Ok(Value::Number(l + r)),
                    Self::Sub => Ok(Value::Number(l - r)),
                    Self::Mul => Ok(Value::Number(l * r)),
                    Self::Div => {
                        if *r != 0 {
                            Ok(Value::Number(l / r))
                        } else {
                            Err(anyhow!("division by zero"))
                        }
                    }
                }
            } else {
                Err(anyhow!("{self} expects right hand side to be a number"))
            }
        } else {
            Err(anyhow!("{self} expects left hand side to be a number"))
        }
    }

    // Return in string to match plait output
    fn show_plait(self) -> &'static str {
        match self {
            Self::Add => "addE",
            Self::Sub => "subE",
            Self::Mul => "mulE",
            Self::Div => "divE",
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Add => '+',
                Self::Sub => '-',
                Self::Mul => '*',
                Self::Div => '/',
            }
        )
    }
}

/// Expressions
//
// Parsed AST

#[derive(Clone, Debug, Eq, PartialEq)]
enum Exp {
    Number(i32),
    Boolean(bool),
    Variable(Symbol),
    If(Box<Exp>, Box<Exp>, Box<Exp>),
    Let1(Symbol, Box<Exp>, Box<Exp>),
    Lambda(Symbol, Box<Exp>),
    Apply(Box<Exp>, Box<Exp>),
    BinFn(BinOp, Box<Exp>, Box<Exp>),
}

impl Exp {
    /// Provide output in plait-style to facilitate testing
    fn show_plait(&self) -> String {
        match self {
            Self::Number(i) => format!("(numE {i})"),
            Self::Boolean(true) => "(boolE #t)".to_owned(),
            Self::Boolean(false) => "(boolE #f)".to_owned(),
            Self::Variable(s) => format!("(varE '{s})"),
            Self::If(c, a, b) => format!(
                "(ifE {} {} {})",
                c.show_plait(),
                a.show_plait(),
                b.show_plait()
            ),
            Self::Let1(s, a, b) => format!("(let1E '{s} {} {})", a.show_plait(), b.show_plait()),
            Self::Lambda(s, b) => format!("(lamE '{s} {})", b.show_plait()),
            Self::Apply(f, b) => format!("(appE {} {})", f.show_plait(), b.show_plait()),
            Self::BinFn(op, l, r) => format!(
                "({} {} {})",
                op.show_plait(),
                l.show_plait(),
                r.show_plait()
            ),
        }
    }
}

// Parse expression from s-expression
fn parse_sexp(sexp: &SExp) -> Result<Exp> {
    match sexp {
        SExp::Number(n) => Ok(Exp::Number(*n)),

        SExp::Boolean(b) => Ok(Exp::Boolean(*b)),

        SExp::Symbol(s) => Ok(Exp::Variable(s.clone())),

        SExp::List(v) => match &v[..] {
            // Form with three arguments
            [SExp::Symbol(f), a, b, c] => {
                if f == "if" {
                    Ok(Exp::If(
                        Box::new(parse_sexp(a)?),
                        Box::new(parse_sexp(b)?),
                        Box::new(parse_sexp(c)?),
                    ))
                } else {
                    Err(anyhow!("Unrecognized form of 4 expressions: {sexp:?}"))
                }
            }

            // Form with two arguments
            [SExp::Symbol(f), a, b] => match f.as_str() {
                "+" => Ok(Exp::BinFn(
                    BinOp::Add,
                    Box::new(parse_sexp(a)?),
                    Box::new(parse_sexp(b)?),
                )),
                "-" => Ok(Exp::BinFn(
                    BinOp::Sub,
                    Box::new(parse_sexp(a)?),
                    Box::new(parse_sexp(b)?),
                )),
                "*" => Ok(Exp::BinFn(
                    BinOp::Mul,
                    Box::new(parse_sexp(a)?),
                    Box::new(parse_sexp(b)?),
                )),
                "/" => Ok(Exp::BinFn(
                    BinOp::Div,
                    Box::new(parse_sexp(a)?),
                    Box::new(parse_sexp(b)?),
                )),
                "lam" => {
                    if let SExp::Symbol(variable) = a {
                        Ok(Exp::Lambda(variable.clone(), Box::new(parse_sexp(b)?)))
                    } else {
                        Err(anyhow!("unrecognized lam form in list: {sexp:?}"))
                    }
                }
                "let1" => {
                    if let SExp::List(assign) = a
                        && let [variable_sexp, value] = &assign[..]
                        && let SExp::Symbol(variable) = variable_sexp
                    {
                        Ok(Exp::Let1(
                            variable.clone(),
                            Box::new(parse_sexp(value)?),
                            Box::new(parse_sexp(b)?),
                        ))
                    } else {
                        Err(anyhow!("unrecognized let1 form in list: {sexp:?}"))
                    }
                }
                _ => Err(anyhow!(
                    "unrecognized symbol in list of 3 expressions: {sexp:?}"
                )),
            },

            // Form with one argument
            [f, a] => Ok(Exp::Apply(
                Box::new(parse_sexp(f)?),
                Box::new(parse_sexp(a)?),
            )),

            _ => Err(anyhow!("unrecognized form in s-expression: {sexp:?}")),
        },
    }
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

/// Values
//
// Results of evaluations

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    Number(i32),
    Boolean(bool),
    Closure(Symbol, Exp, Env),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            Self::Number(i) => &i.to_string(),
            Self::Boolean(true) => "#t",
            Self::Boolean(false) => "#f",
            Self::Closure(..) => "#closure",
        };
        write!(f, "{s}")
    }
}

/// Env is an immutable hash map
type Env = HashMap<Symbol, Value>;

/// Interpret from an empty environment
fn calc(exp: &Exp) -> Result<Value> {
    interp(exp, &Env::new())
}

/// Interpret an expression
fn interp(exp: &Exp, nv: &Env) -> Result<Value> {
    match exp {
        Exp::Number(i) => Ok(Value::Number(*i)),

        Exp::Boolean(b) => Ok(Value::Boolean(*b)),

        Exp::BinFn(op, l, r) => op.apply(&interp(l, nv)?, &interp(r, nv)?),

        Exp::Lambda(var, body) => Ok(Value::Closure(var.clone(), *body.clone(), nv.clone())),

        Exp::Variable(v) => if let Some(x) = nv.get(v) {
            Ok(x.clone())
        } else {
            Err(anyhow!("{v} not bound"))
        }

        Exp::If(c, t, e) => if let Value::Boolean(b) = interp(c, nv)? {
            if b {
                interp(t, nv)
            } else {
                interp(e, nv)
            }
        } else {
            Err(anyhow!("expects conditional to evaluate to a boolean"))
        }

        Exp::Let1(var, val, body) => {
            let new_nv = nv.update(var.clone(), interp(val, nv)?);
            interp(body, &new_nv)
        }

        Exp::Apply(fun, arg) => {
            let arg = interp(arg, nv)?;
            if let Value::Closure(var, body, cnv) = interp(fun, nv)? {
                let new_cnv = cnv.update(var.clone(), arg);
                interp(&body, &new_cnv)
            } else {
                Err(anyhow!("app didnt get a closure"))
            }
        }
    }
}

/// Parse, evaluate and convert to a string
pub fn run(s: &str) -> Result<String> {
    let (sexp, rest) = read_sexp(s)?;
    if !rest.trim().is_empty() {
        bail!("Leftover string in parse");
    }
    let exp = parse_sexp(&sexp)?;
    let value = calc(&exp)?;
    Ok(value.to_string())
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
        assert_eq!(
            sexp,
            SExp::List(vec![
                SExp::Symbol("*".to_string()),
                SExp::Number(2),
                SExp::Number(3)
            ])
        );
        let exp = parse_sexp(&sexp).unwrap();
        assert_eq!(
            exp,
            Exp::BinFn(BinOp::Mul, Box::new(Exp::Number(2)), Box::new(Exp::Number(3)))
        );
    }

    #[test]
    fn test_parse_binary_add() {
        let (sexp, rest) = read_sexp("{+ 22 33}").unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            sexp,
            SExp::List(vec![
                SExp::Symbol("+".to_string()),
                SExp::Number(22),
                SExp::Number(33)
            ])
        );
        let exp = parse_sexp(&sexp).unwrap();
        assert_eq!(
            exp,
            Exp::BinFn(BinOp::Add, Box::new(Exp::Number(22)), Box::new(Exp::Number(33)))
        );
    }

    #[test]
    fn test_parse_if() {
        let (sexp, rest) = read_sexp("{if x 2 {+ y 4}}").unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            sexp,
            SExp::List(vec![
                SExp::Symbol("if".to_string()),
                SExp::Symbol("x".to_string()),
                SExp::Number(2),
                SExp::List(vec![
                    SExp::Symbol("+".to_string()),
                    SExp::Symbol("y".to_string()),
                    SExp::Number(4)
                ])
            ])
        );
        let exp = parse_sexp(&sexp).unwrap();
        assert_eq!(
            exp,
            Exp::If(
                Box::new(Exp::Variable("x".to_string())),
                Box::new(Exp::Number(2)),
                Box::new(Exp::BinFn(
                    BinOp::Add,
                    Box::new(Exp::Variable("y".to_string())),
                    Box::new(Exp::Number(4))
                ))
            )
        );
    }
}
