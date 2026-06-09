"""Python version of PLAI evaluation."""

import re
from copy import deepcopy
from dataclasses import dataclass
from typing import Final, Literal, assert_never


class EvalError(Exception):
    """Exceptions in evaluation code."""


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
                raise EvalError("Unexpected tag")
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
        case [str(_), _, _]:
            raise EvalError("Unrecognized symbol")
        case [f, a]:
            return ApplyE(parse_sexp(f), parse_sexp(a))
        case _:
            raise EvalError("Unrecognized form")


def parse(s: str) -> str:
    """Parse from string to an expression and convert to plait-style."""
    return show_plait(parse_sexp(read_sexp(s)))


def test_parse_sexp() -> None:
    """Test parse_sexp."""

    assert parse_sexp(1) == 1
    assert parse_sexp(["if", 2, 3, 4]) == IfE(2, 3, 4)
    assert parse_sexp(["+", 2, 3]) == BinFnE("+", 2, 3)
    assert parse_sexp(["+", ["*", 3, 4], 5]) == BinFnE("+", BinFnE("*", 3, 4), 5)


#
# Values
#

# Need to take case as bool is a subclass of int
type Value = int | bool | Closure


@dataclass(frozen=True)
class Closure:
    """Closure value."""

    variable: Symbol
    body: Exp
    env: Env


type Env = dict[Symbol, Value]


def show_value(value: Value) -> str:
    """Return string form of the value."""

    match value:
        case True:
            return "#t"
        case False:
            return "#f"
        case int(i):
            return str(i)
        case Closure(_, _, _):
            return "#closure"
        case _ as unreachable:
            assert_never(unreachable)


def calc(exp: Exp) -> Value:
    """Interpret and expression with an empty environment."""

    return interp(exp, {})


def interp(exp: Exp, nv: Env) -> Value:
    """Interpret an expression."""

    match exp:
        case bool(b):
            return b

        case int(i):
            return i

        case str(s):
            try:
                return nv[s]
            except KeyError as err:
                raise EvalError(f"{s}: not bound") from err

        case ApplyE(func, arg):
            match interp(func, nv):
                case Closure(var, body, cnv):
                    new_cnv = deepcopy(cnv)
                    new_cnv[var] = interp(arg, nv)
                    return interp(body, new_cnv)
                case bool(_) | int(_):
                    raise EvalError("applying a non-closure")

        case BinFnE(op, left, right):
            left_value = interp(left, nv)
            right_value = interp(right, nv)
            if isinstance(right_value, bool) or not isinstance(right_value, int):
                raise EvalError(f"{op} expects right hand side to be a number")
            if isinstance(left_value, bool) or not isinstance(left_value, int):
                raise EvalError(f"{op} expects left hand side to be a number")
            match op:
                case "+":
                    return left_value + right_value
                case "-":
                    return left_value - right_value
                case "*":
                    return left_value * right_value
                case "/":
                    if right_value == 0:
                        raise EvalError("division by zero")
                    return left_value // right_value
        #                case _ as unreachable:
        #                    assert_never(unreachable)

        case IfE(condition, then_, else_):
            condition_value = interp(condition, nv)
            if not isinstance(condition_value, bool):
                raise EvalError("'if' expects conditional to evaluate to a boolean")
            if condition_value:
                return interp(then_, nv)
            return interp(else_, nv)

        case LambdaE(variable, body):
            return Closure(variable, body, nv)

        case Let1E(variable, value, body):
            new_nv = deepcopy(nv)
            new_nv[variable] = interp(value, nv)
            return interp(body, new_nv)

        case _ as unreachable:
            assert_never(unreachable)

    return 0


def run(s: str) -> str:
    """Parse, evaluate and convert to a string."""

    return show_value(calc(parse_sexp(read_sexp(s))))
