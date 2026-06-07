"""Python version of PLAI evaluation."""

import re
from dataclasses import dataclass
from typing import Final, Literal, assert_never

#
# S-Expression
#

type SExp = int | bool | str | list[SExp]

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

    stack: list[list[SExp]] = [[]]
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
    return stack[0][0]


def test_read_sexp() -> None:
    """Test read_sexp."""

    assert read_sexp("23") == 23
    assert read_sexp("{1 x 3 cat #false}") == [1, "x", 3, "cat", False]
    assert read_sexp("{1 {2 #t}}") == [1, [2, True]]
    assert read_sexp("{1 {2 3 {4 5}}}") == [1, [2, 3, [4, 5]]]
    assert read_sexp("{+ {- 2 3} {* 4 5}}") == ["+", ["-", 2, 3], ["*", 4, 5]]


#
# Expressions
#


type Symbol = str
type Exp = int | bool | Symbol | ApplyE | BinFnE | IfE | LambdaE | Let1E


@dataclass(frozen=True)
class ApplyE:
    """Apply Expression."""

    function: Exp
    argument: Exp


@dataclass(frozen=True)
class BinFnE:
    """If Expression."""

    operator: Literal["+", "-", "*", "/"]
    left: Exp
    right: Exp


@dataclass(frozen=True)
class IfE:
    """If Expression."""

    condition: Exp
    then_: Exp
    else_: Exp


@dataclass(frozen=True)
class LambdaE:
    """Lambda Expression."""

    variable: Symbol
    body: Exp


@dataclass(frozen=True)
class Let1E:
    """Let1 Expression."""

    variable: Symbol
    value: Exp
    body: Exp


BinOpNames: Final[dict[str, str]] = {"+": "addE", "-": "subE", "*": "mulE", "/": "divE"}


def show_plait(exp: Exp) -> str:
    """Provide output in plait-style to facilitate testing."""

    match exp:
        case True:
            return "(boolE #t)"
        case False:
            return "(boolE #f)"
        case int(i):
            return f"(numE {i})"
        case str(s):
            return f"(varE '{s})"
        case BinFnE(op, l, r):
            return f"({BinOpNames[op]} {show_plait(l)} {show_plait(r)})"
        case IfE(c, t, e):
            return f"(ifE {show_plait(c)} {show_plait(t)} {show_plait(e)})"
        case Let1E(s, a, b):
            return f"(let1E '{s} {show_plait(a)} {show_plait(b)})"
        case LambdaE(v, b):
            return f"(lamE '{v} {show_plait(b)})"
        case ApplyE(f, b):
            return f"(appE {show_plait(f)} {show_plait(b)})"
        case _ as unreachable:
            assert_never(unreachable)


def parse_sexp(s: SExp) -> Exp:
    """Parse an s-expression to an expression."""

    match s:
        case int(_) | str(_):
            return s
        case ["if", condition, then_, else_]:
            return IfE(parse_sexp(condition), parse_sexp(then_), parse_sexp(else_))
        case ["+" | "-" | "*" | "/" as op, left, right]:
            return BinFnE(op, parse_sexp(left), parse_sexp(right))
        case ["lam", str(variable), body]:
            return LambdaE(variable, parse_sexp(body))
        case ["let1", [str(variable), value], body]:
            return Let1E(variable, parse_sexp(value), parse_sexp(body))
        case [str(s), _, _]:
            raise ValueError("Unrecognized symbol")
        case [f, a]:
            return ApplyE(parse_sexp(f), parse_sexp(a))
        case _:
            raise ValueError("Unrecognized form")


def parse(s: str) -> str:
    """Parse from string to an expression and convert to plait-style."""
    return show_plait(parse_sexp(read_sexp(s)))


def test_parse_sexp() -> None:
    """Test parse_sexp."""

    assert parse_sexp(1) == 1
    assert parse_sexp(["if", 2, 3, 4]) == IfE(2, 3, 4)
    assert parse_sexp(["+", 2, 3]) == BinFnE("+", 2, 3)
    assert parse_sexp(["+", ["*", 3, 4], 5]) == BinFnE("+", BinFnE("*", 3, 4), 5)
