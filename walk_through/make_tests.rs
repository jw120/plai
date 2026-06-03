use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Result};

#[derive(Clone, Copy, Debug)]
enum Language {
    Plait,
    Python,
    Rust,
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
            Long("language") => {
                language = match parser.value()?.string()?.to_lowercase().as_str() {
                    "plait" => Some(Language::Plait),
                    "python" => Some(Language::Python),
                    "rust" => Some(Language::Rust),
                    _ => bail!("Unknown language"),
                }
            }
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
        Language::Plait => Ok("#lang plait\n\
                              (print-only-errors #true)\n\n\
                              (require \"evaluation.rkt\")\n\n"),
        _ => Err(anyhow!("NYI")),
    }
}

fn data_lines(_language: Language, input: &File, output: &mut File) -> Result<()> {
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
        let command = if parts[2].starts_with('"') {
            "test/exn"
        } else {
            "test"
        };
        writeln!(
            output,
            "({} ({} {}) {})",
            command, parts[0], parts[1], parts[2]
        )?;
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

    Ok(())
}
