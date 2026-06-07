"""Python version of PLAI evaluation."""

import re
from typing import Final

#
# S-Expression
#

type SExp = list[int | bool | str | SExp]

# Regular expression for terms in the s-expression.
#
# Alternatives are tagged with group names
# Borrowed from https://rosettacode.org/wiki/S-expressions#Python
TERM_REGEXP: Final[re.Pattern[str]] = re.compile(
    r"""\s*
        (?:
            (?P<open>    {        ) |
            (?P<close>   }        ) |
            (?P<number>  \-?\d+    ) |
            (?P<boolean> \#(true|false|t|f) ) |
            (?P<symbol>  [^{^}\s]+ )
         )""",
    re.VERBOSE,
)


def read_sexp(s: str) -> SExp:
    """Read an s-expression from a string."""

    stack: list[SExp] = []
    for match in re.finditer(TERM_REGEXP, s):
        named_subgroups: list[tuple[str, str]] = [
            (tag, value) for tag, value in match.groupdict().items() if value is not None
        ]
        assert len(named_subgroups) == 1
        tag, value = named_subgroups[0]
        match tag:
            case "open":
                stack.append([])
            case "close":
                if len(stack) > 1:
                    top = stack.pop()
                    stack[-1].append(top)
            case "boolean":
                stack[-1].append(value[1] == "t")
            case "number":
                stack[-1].append(int(value))
            case "symbol":
                stack[-1].append(value)
            case _:
                raise ValueError("Unexpected tag")
    assert len(stack) == 1, "Mismatched braces"
    return stack[0]


def test_read_sexp() -> None:
    """Test read_sexp."""

    assert read_sexp("{1 x 3 cat #false}") == [1, "x", 3, "cat", False]
    assert read_sexp("{1 {2 #t}}") == [1, [2, True]]
    assert read_sexp("{1 {2 3 {4 5}}}") == [1, [2, 3, [4, 5]]]
    assert read_sexp("{+ {- 2 3} {* 4 5}}") == ["+", ["-", 2, 3], ["*", 4, 5]]


#
# Expressions
#

# type Expression =
