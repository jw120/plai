use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use anyhow::{Context, Ok, Result, anyhow, bail};

#[derive(Clone, Copy, Debug)]
enum Language {
    Plait,
    Python,
    Rust,
}

impl Language {
    fn parse(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "plait" => Ok(Self::Plait),
            "python" => Ok(Self::Python),
            "rust" => Ok(Self::Rust),
            _ => Err(anyhow!("Unknown language: '{s}'")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Test {
    Parse,
    Run,
}

impl Test {
    fn parse(s: &str) -> Result<Self> {
        match s {
            "parse" => Ok(Self::Parse),
            "run" => Ok(Self::Run),
            _ => Err(anyhow!("Unknown test command: '{s}'")),
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Parse => "parse",
            Self::Run => "run",
        }
    }
}

#[derive(Debug)]
struct Args {
    language: Language,
    input: PathBuf,
    output: PathBuf,
}

fn parse_args() -> Result<Args> {
    use lexopt::prelude::*;

    let mut language: Option<Language> = None;
    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("language") => language = Some(Language::parse(&parser.value()?.string()?)?),
            Long("input") => input = Some(PathBuf::from(parser.value()?.string()?)),
            Long("output") => output = Some(PathBuf::from(parser.value()?.string()?)),
            Long("help") => {
                println!(
                    "Usage: make_tests --language=[plait|rust|python] --input=FILE --output=FILE"
                );
                std::process::exit(0);
            }
            _ => return Err(anyhow::Error::from(arg.unexpected())),
        }
    }

    Ok(Args {
        language: language.context("missing language argument")?,
        input: input.context("missing input argument")?,
        output: output.context("missing output argument")?,
    })
}

fn header(language: Language) -> Result<&'static str> {
    match language {
        Language::Plait => Ok(r#"
#lang plait

(print-only-errors #true)
(require "evaluation.rkt")

"#),
        Language::Rust => Ok("mod evaluation;\n\n\
                              #[cfg(test)]\n\n\
                              mod tests {\n\n\
                              use crate::evaluation::*;\n\n\
                              #[test]\n\
                              fn test_all() {\n"),

        Language::Python => Err(anyhow!("NYI")),
    }
}

fn footer(language: Language) -> Result<&'static str> {
    match language {
        Language::Plait => Ok(""),
        Language::Rust => Ok("    }\n}\nfn main() { }\n"),
        Language::Python => Err(anyhow!("NYI")),
    }
}

fn line_plait(test: Test, exception: bool, input: &str, expected: &str) -> String {
    let test_function = if exception { "test/exn" } else { "test" };
    let quote_output = match test {
        Test::Parse => exception,
        Test::Run => true,
    };
    let expected = if quote_output {
        &format!("\"{expected}\"")
    } else {
        expected
    };
    format!(
        "({test_function} ({} `{input}) {expected})\n",
        test.to_str()
    )
}

fn line_rust(test: Test, exception: bool, input: &str, expected: &str) -> String {
    if test == Test::Run || exception {
        return String::new();
    }
    format!(
        "    assert_eq!({}(\"{input}\").unwrap(), \"{expected}\");\n",
        test.to_str()
    )
}

fn data_lines(language: Language, input: &File, output: &mut File) -> Result<()> {
    let reader = BufReader::new(input);
    for (line_number, line) in reader.lines().enumerate() {
        let line_number = line_number + 1;
        let line = line?;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            bail!("failed at line {line_number}: {line}");
        }
        let test = Test::parse(parts[0])?;
        let exception;
        let expected;
        if let Some(rest) = parts[2].strip_prefix("EXCEPTION:") {
            exception = true;
            expected = rest;
        } else {
            exception = false;
            expected = parts[2];
        }
        let output_line = match language {
            Language::Plait => line_plait(test, exception, parts[1], expected),
            Language::Rust => line_rust(test, exception, parts[1], expected),
            Language::Python => bail!("NYI"),
        };
        output.write_all(output_line.as_bytes())?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = parse_args()?;

    let input_file = OpenOptions::new().read(true).open(args.input)?;

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(args.output)?;

    output_file.write_all(header(args.language)?.as_bytes())?;
    data_lines(args.language, &input_file, &mut output_file)?;
    output_file.write_all(footer(args.language)?.as_bytes())?;

    Ok(())
}
