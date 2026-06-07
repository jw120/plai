.PHONY: all check clean plait python rust

# Main targets run our tests (in the source file and from our generated test file)

all: plait python rust

plait: evaluation_test.rkt
	raco test $< evaluation.rkt

python: evaluation_test.py
	uv run pytest evaluation.py $<

rust: evaluation_test.rs
	cargo test

# Generated test files from common test definitions

evaluation_test.rkt: test_definitions.csv make_tests.rs
	cargo run --bin=make_tests -- --input=$< --language=plait --output=$@

evaluation_test.py: test_definitions.csv make_tests.rs
	cargo run --bin=make_tests -- --input=$< --language=python --output=$@

evaluation_test.rs: test_definitions.csv make_tests.rs
	cargo run --bin=make_tests -- --input=$< --language=rust --output=$@

# Formatting and linting

check: plait-check python-check rust-check

plait-check:

python-check:
	ruff format evaluation.py
	ty check evaluation.py
	ruff check evaluation.py

rust-check:
	cargo fmt
	cargo clippy

# Remove generated test files

clean:
	-rm evaluation_test.rkt evlauation_test.rs evaluation_test.py
