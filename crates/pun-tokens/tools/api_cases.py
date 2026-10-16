# Generates tests/api_cases.json: python3 tools/api_cases.py > tests/api_cases.json
import json, sys
W = ["T_WHITESPACE"]; C = ["T_COMMENT"]; D = ["T_DOC_COMMENT"]
def each(*names, args=()):
    return [["each", n, *args] for n in names]
QUERY = ["tok", "nextMeaningful", "prevMeaningful", "nextNonWs", "prevNonWs", "detect", "emptyAt", "isWs", "isArray", "superGlobal",
         "unaryPred", "unarySucc", "binary", "lambda", "anonClass", "methodAttributes", "classyModifiers", "arrayMultiLine",
         "blockMultiline", "arrowEnd", "doWhile", "enumCase", "blocks"]
cases = []
def case(code, *ops):
    cases.append({"code": code, "ops": [o for group in ops for o in (group if group and isinstance(group[0], list) else [group])]})

A = """<?php
namespace A\\B;
use B\\C, D;
use function f;
/** doc */
#[Attr(1)]
final readonly class K extends P implements I {
    use T;
    const X = [1, 2], Y = array(3,
        4);
    public static function m(&$a, int ...$b): ?int { return $a ?? -$b[0] + +1 - ~$a & $b; }
    abstract protected function n();
    final public function &r() { return $this->p; }
    private $p = 1;
    public function __construct(public readonly int $q, protected $w, private(set) string $e = '', final $f = 1) {}
}
abstract class L {}
$x = fn($y) => [$y, ($y), [1 => 2]];
$z = function () use ($x) { return $x[0] ?? null; };
$i++; --$i; $i--; $o->p++; $a[1]--; ${'v'}++; $o->{'p'}--;
$s = $_GET['a'] . $GLOBALS['b'] . $_server;
$n = new class(1, new class {}) extends L { public $z; };
$m = new #[A] readonly class {};
do { echo 1; } while (0);
while (0) {}
if (1) { } while (0);
enum E: string { case A = 'a'; public function f() { switch (1) { case 1: break; } } }
switch ($x) { case 1: }
$b = !$a && @f() || $a and $b or $c xor $d;
$c = $a <=> $b ?? $c ?: $d; $a .= 1; $a **= 2; $a ??= 3; $r = &$a; f(...$a); $a = $b & $c; $q = "x" . `y` . __LINE__ - 1;
function &g(array &$a) { return $a; }
$h = fn&($a) => $a;
$k = static fn(array $a): array => array_map(fn($b) => $b + 1, $a);
$l = [fn() => 1, fn() => [2], fn() => (3)];
"""
case(A, each(*QUERY),
     each("nonWsSibling", args=(1, " \n")), each("nonWsSibling", args=(-1, " ")), each("nonEmptySibling", args=(1,)),
     each("meaningfulSibling", args=(-1,)), each("notOfKindsSibling", args=(1, ["T_WHITESPACE", "T_VARIABLE"])),
     each("notOfKindSibling", args=(-1, ["(", W, [ "T_STRING", "m"]])), each("isWs", args=(" ",)),
     each("equals", args=(["T_STRING", "k"], False)), each("equals", args=(["T_STRING", "k"],)), each("equals", args=("&",)),
     each("equalsAny", args=([["T_VARIABLE", "$A"], "(", ["T_FUNCTION"]], False)),
     each("isGivenKind", args=(["T_CLASS", "T_FN"],)), each("isGivenKind", args=("T_VARIABLE",)),
     each("nextOfKind", args=([";", ["T_STRING", "C"]], False)), each("prevOfKind", args=([["T_FUNCTION"], "{"],)),
     each("ofKindSibling", args=(1, [["T_NOT_A_TOKEN_IN_FILE"]] if False else [["T_GOTO"]])),
     [["classyElements"], ["importUses", False], ["importUses", True], ["givenKind", "T_VARIABLE"], ["givenKind", "T_GOTO"],
      ["givenKind", ["T_FUNCTION", "T_CLASS", "T_GOTO"], 10, 200], ["givenKind", ["T_FN"], 5], ["found", ["T_FN"]], ["found", "{"],
      ["found", "nope"], ["countKind", ["T_VARIABLE"]], ["countKind", "("], ["allFound", ["T_FN", "T_CLASS"]],
      ["allFound", ["T_FN", "T_GOTO"]], ["anyFound", ["T_GOTO", "T_FN"]], ["anyFound", ["T_GOTO"]], ["monolithic"], ["altSyntax"],
      ["multiline", 0, 0], ["multiline", 1, 3], ["count"], ["partial", 1, 6],
      ["sequence", [["T_FUNCTION"], ["T_STRING", "M"], "("], 0, None, False], ["sequence", [["T_FUNCTION"], ["T_STRING", "M"], "("]],
      ["sequence", [["T_FUNCTION"], ["T_STRING", "M"]], 0, None, {"1": False}], ["sequence", [["T_FUNCTION"], ["T_STRING", "M"]], 0, None, {"0": False}],
      ["sequence", [["T_USE"], ["T_STRING"]], 30], ["sequence", [["T_USE"], ["T_STRING"]], 0, 3], ["sequence", [["T_USE"], ["T_STRING"]], 0, 9999],
      ["sequence", [["T_USE"], ["T_GOTO"]]], ["sequence", [["T_RETURN"], ["T_VARIABLE"], "?"]], ["sequence", [["T_RETURN"], ";"]],
      ["sequence", []], ["sequence", [W]], ["sequence", [["T_COMMENT"]]], ["sequence", [""]], ["sequence", [["T_STRING", ""]]],
      ["sequence", ["{", "}"], 5000],
      ["blockEnd", 0, 1], ["blockEnd", 16, 1], ["blockEnd", 1, 1], ["blockStart", 2, 1]])

# Every block type, both directions.
B = """<?php
use A\\{B, C};
[$a, [$b]] = $c; list($d) = [1, [2]]; $e[1]; $o->{'p'}; ${'v'}; A::{'X'}; "${x} {$y}";
#[Attr([1])] function f((A&B)|null $x, (C&D)|E $y) {}
$n = new A(1); $m = new class() {};
class P { public int $p { get => 1; set { $this->p = $value; } } public $q { #[A] get; } }
"""
case(B, each("blocks"), each("detect"), each("blockMultiline"), each("arrayMultiLine"), [["classyElements"]])

# Multi-namespace imports.
case("<?php namespace A; use X; namespace B { use Y; use Z; } namespace { use W; }",
     [["importUses", False], ["importUses", True]])
case("<?php use X; class A { use T; } $f = function () use ($a) {};", [["importUses", False], ["importUses", True]])
case("<?php echo 1;", [["importUses", False], ["importUses", True], ["classyElements"]])

# Monolithic / shebang / inline html.
for code in ["<?php echo 1;", "x<?php echo 1;", "#!/usr/bin/env php\n<?php echo 1;", "#!\n<?php echo 1;", "#!/a\nb\n<?php 1;", "<?= 1;",
             "<?php 1; ?>x<?php 2;", "<?php 1; ?>\n", "x", "", "<?php if (1): endif;", "<?php while (1): endwhile;",
             "<?php for (;;): endfor;", "<?php foreach ($a as $b): endforeach;", "<?php switch (1): endswitch;",
             "<?php declare(ticks=1): enddeclare;"]:
    case(code, [["monolithic"], ["altSyntax"], ["count"]])

# Mutations.
M = "<?php\n$a = 1; // c\n    $b = 2; /* d */\n\n$c = 3;\n"
case(M, [["insertAt", 1, [["T_VARIABLE", "$x"], ";"]], ["countKind", ["T_VARIABLE"]], ["countKind", ";"], ["changed"]])
case(M, [["insertAt", 1, []], ["changed"]])
cases.append({"code": M, "ops": [["insertAt", 1, [""]]], "noTail": True})
cases.append({"code": M, "ops": [["insertAt", 9999, ["x"]]], "noTail": True})  # upstream resizes before throwing: state is garbage
case(M, [["insertSlices", {"1": [["T_VARIABLE", "$x"]], "3": [["T_WHITESPACE", " "]], "5": []}], ["found", ["T_VARIABLE"]]])
case(M, [["insertSlices", {}], ["changed"]])
case(M, [["set", 1, ["T_VARIABLE", "$a"]], ["changed"], ["set", 1, ["T_VARIABLE", "$z"]], ["changed"], ["countKind", ["T_VARIABLE"]],
         ["set", 2, " "], ["countKind", " "], ["found", ["T_WHITESPACE"]]])
case(M, [["clearChanged"], ["changed"], ["clearAt", 3], ["changed"], ["emptyAt", 3], ["found", ""], ["countKind", ""], ["clearEmpty"],
         ["found", ""], ["count"]])
case(M, [["clearEmpty"], ["count"]])
case(M, [["overrideRange", 1, 3, [["T_VARIABLE", "$q"]]], ["overrideRange", 1, 1, [["T_VARIABLE", "$r"], "=", ["T_LNUMBER", "2"], ";"]],
         ["overrideRange", 5, 6, [["T_LNUMBER", "5"], ";"]], ["clearEmpty"], ["count"]])
case(M, [["clearRange", 1, 4], ["clearEmpty"], ["count"]])
for i in range(1, 30):
    case(M, [["clearMerge", i]])
for i in range(0, 22):
    case(M, [["removeLeadingWs", i], ["removeTrailingWs", i]])
    case(M, [["removeTrailingWs", i, " "]])
case("<?php $a; // c\r\n  $b; # d\n$c; // e\n$d; //\n\n $e; # f\n", each("removeTrailingWs"))
case("<?php $a; // c\r\n  $b; # d\n$c; // e\n$d; //\n\n $e;", [["each", "removeTrailingWs", "\n"]])
case("<?php $a;", [["fromArray", [["T_COMMENT", "// c"], ["T_WHITESPACE", " "], ";"]], ["removeTrailingWs", 0]])
case("<?php // c\n", [["removeTrailingWs", 0], ["removeTrailingWs", 1], ["removeLeadingWs", 3]])
for code in ["<?php\n$a;", "<?php $a;", "<?php\r\n$a;", "<?php  $a;", "<?php\n\n$a;"]:
    for args in [(1, 0, " "), (1, 0, "\n"), (1, 0, ""), (1, 1, "\n  "), (1, 1, "\r\n  "), (0, 1, "\n"), (0, 1, "\r\n"), (0, 1, " "),
                 (0, 0, " "), (2, 0, " "), (2, 1, "  "), (2, 1, "")]:
        case(code, [["ensureWs", *args], ["count"]])
case("<?php $a;", [["setCode", "<?php $a;"], ["changed"], ["setCode", "<?php $b;"], ["changed"], ["count"], ["found", ["T_VARIABLE"]]])
case("<?php $a;", [["fromArray", [["T_OPEN_TAG", "<?php "], "(", ["T_VARIABLE", "$a"], ""]], ["changed"], ["count"], ["found", "("],
                   ["found", ["T_VARIABLE"]], ["blockEnd", 1, 1], ["clearEmpty"], ["count"]])
case("<?php $a;", [["fromArray", [")", "(", "("]], ["blockStart", 1, 0], ["blockEnd", 1, 2]])
case("<?php $a;", [["fromArray", []], ["count"], ["sequence", [";"]], ["code"]])
case("<?php f(1)(2);", [["blockEnd", 1, 2], ["blockEnd", 1, 2], ["blockStart", 1, 4], ["clearAt", 2], ["blockEnd", 1, 5],
                        ["insertAt", 0, ["x"]], ["blockEnd", 1, 6], ["blockEnd", 1, 6], ["clearEmpty"]])
case("<?php f(1)(2);", [["blockEnd", 1, 2], ["blockEnd", 1, 5], ["clearAt", 1], ["clearAt", 0], ["clearEmpty"], ["blockStart", 1, 2],
                        ["blockStart", 1, 5], ["blockEnd", 1, 3], ["code"]])
case("<?php f(1)(2);", [["blockEnd", 1, 2], ["set", 2, "("], ["set", 4, ["T_STRING", "x"]], ["blockEnd", 1, 2]])

# Token-level equality, incl. the "&" special cases.
AMP = [["T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG", "&"], ["T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG", "&"]]
toks_ = ["&", "(", ["T_STRING", "Foo"], ["T_STRING", "&"], *AMP]
protos_ = ["&", "(", "Foo", ["T_STRING"], ["T_STRING", "foo"], ["T_STRING", "Foo"], ["T_VARIABLE", "Foo"], *AMP]
ops = []
for a in toks_:
    for p in protos_:
        ops += [["newEquals", a, p], ["newEquals", a, p, False]]
    for b in toks_:
        ops += [["newEqualsToken", a, b], ["newEqualsToken", a, b, False]]
case("<?php function g(&$a) {} f($b & $c); $x = 'a';", ops, each("equalsToken", args=(3,)), each("equalsToken", args=(9, False)))
# Token predicates over every kind of token (casts, magic/native constants, keywords, object operators).
case("<?php (int)$a; (bool)$a; (float)$a; (string)$a; (array)$a; (object)$a; (unset)$a; (void)f(); __CLASS__; __DIR__; __FILE__;"
     " __FUNCTION__; __LINE__; __METHOD__; __NAMESPACE__; __TRAIT__; __PROPERTY__; true; FALSE; Null; \\null; $a->b; $a?->b; A::b;"
     " trait T {} interface I {} enum E {} goto x; x: yield 1; yield from []; match(1) { default => 1 }; # c\n /** d */ ?>x",
     each("tok"), each("isWs", args=(" \n",)))

# Empty (cleared) tokens are skipped by the sibling helpers.
case(M, [["clearAt", 2], ["clearAt", 3], ["clearAt", 5]], each("nonEmptySibling", args=(1,)), each("nonEmptySibling", args=(-1,)),
     each("nextMeaningful"), each("notOfKindsSibling", args=(-1, ["T_LNUMBER"])), each("emptyAt"))
case("<?php $a = 1;", [["sequence", [["T_VARIABLE"], "="], 0, 1], ["sequence", [["T_VARIABLE"], "="], 0, 3], ["sequence", ["=", ["T_LNUMBER"], ";", ";"]]])
case("<?php function f(Foo &$x) {} $a = FOO & $b; fn(Foo &$y) => 1; $c = [Foo &$z];",
     each("unaryPred"), each("binary"), each("equalsProto", args=(3,)), each("equalsProto", args=(7, False)))
case("<?php switch (1) { case 1: }", each("enumCase"))
case("<?php enum E { const X = 1; public function f() { if (1) {} } case B; }", each("enumCase"))
case("""<?php class A { public function __construct($a) {} public function f() { return new class(1, new class {}, (2)) { public $x; }; }
  public function g() { $c = new class { public $y; }; } public $z; }""", [["classyElements"]])
# Transformers on token lists PHP itself would not produce (Transformers::transform after Tokens::fromArray).
O = ["T_OPEN_TAG", "<?php "]
case("<?php", [["fromArray", [O, ["T_STRING", "A"], ["T_DOUBLE_COLON", "::"], "{", ["T_CONSTANT_ENCAPSED_STRING", "'b'"], "}", "(", ")", "{",
                ["T_CONSTANT_ENCAPSED_STRING", "'c'"], "}", ";"]], ["transform"]], each("tok"))
case("<?php", [["fromArray", [O, ["T_STRING", "f"], "(", ")", "{", "}", ";"]], ["transform"]], each("tok"))
case("<?php", [["fromArray", [O, ["T_VARIABLE", "$a"], ["T_OBJECT_OPERATOR", "->"], "{", ["T_STRING", "b"]]], ["transform"]])
case("<?php", [["fromArray", [O, ["T_VARIABLE", "$a"], ["T_OBJECT_OPERATOR", "->"], "{", ["CT::T_CURLY_CLOSE", "}"], ";"]], ["transform"]])
# FoundKey from &str / Kind; Proto from &Token.
case("<?php $a = [1]; f(...);", [["found", "["], ["found", ["CT::T_ARRAY_BRACKET_OPEN"]], ["found", ["CT::T_FIRST_CLASS_CALLABLE"]], ["countKind", "="]],
     each("equalsProto", args=(3,)))

print("[\n" + ",\n".join(json.dumps(c, separators=(",", ":")) for c in cases) + "\n]")
