"""Python version of PLAI evaluation."""

import re
from typing import Any, Final, Pattern

# S-Expression

type SExp = list[int | str | SExp]

# Regular expression for terms in the s-expression.
#
# Alternatives are tagged with group names
# Borrowed from https://rosettacode.org/wiki/S-expressions#Python
TERM_REGEXP: Final[Pattern[str]] = re.compile(
    r"""\s*
        (?:
            (?P<open>   {        ) |
            (?P<close>  }        ) |
            (?P<number> \-?\d+    ) |
            (?P<symbol> [^{^}\s]+ )
         )""",
    re.VERBOSE,
)


def read_sexp(s: str) -> SExp:
    """Read an s-expression from a string."""

    output: SExp = []
    stack: list[SExp] = []
    for match in re.finditer(TERM_REGEXP, s):
        named_subgroups: list[tuple[str, str]] = [
            (tag, value) for tag, value in match.groupdict().items() if value is not None
        ]
        assert len(named_subgroups) == 1
        tag, value = named_subgroups[0]
        print(tag, value, output, stack)
        match tag:
            case "open":
                stack.append(output)
                output = []
            case "close":
                assert stack, "Unexpected close bracket"
                tmp_output, output = output, stack.pop()
                output.append(tmp_output)
            case "number":
                output.append(int(value))
            case "symbol":
                output.append(value)
            case _:
                raise ValueError("Unexpected tag")
    assert not stack, "Missing close bracket"
    assert isinstance(output, list), "Internal failure"
    return output[0]


def test_read_sexp():
    assert read_sexp("{1 x 3 cat}") == [1, "x", 3, "cat"]
    assert read_sexp("{1 {2 3}}") == [1, [2, 3]]
    assert read_sexp("{+ {- 2 3} {* 4 5}}") == ["+", ["-", 2, 3], ["*", 4, 5]]
