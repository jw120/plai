.PHONY: all clean plait rust

all: plait rust

plait: evaluation_test.rkt
	raco test $<

evaluation_test.rkt: test_definitions.csv make_tests.rs
	cargo run --bin=make_tests -- --input=$< --language=plait --output=$@

rust: evaluation_test.rs
	cargo test

evaluation_test.rs: test_definitions.csv make_tests.rs
	cargo run --bin=make_tests -- --input=$< --language=rust --output=$@

clean:
	-rm *_test.rkt *_test.rs
