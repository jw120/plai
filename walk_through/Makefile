.PHONY: all plait

all: plait

plait: evaluation_test.rkt
	raco test evaluation_test.rkt

evaluation_test.rkt: test_definitions.csv make_tests.rs
	cargo run --bin=make_tests -- --input=test_definitions.csv --language=plait --output=$@

clean:
	-rm *_test.rkt
