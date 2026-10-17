#!/usr/bin/env python3
"""Write crates/pun-tokens/tests/fixtures/lint_cases.json: PHP snippets and what real PHP 8.5's
`token_get_all($code, TOKEN_PARSE)` makes of them (null, or PHP-CS-Fixer's "Parse error: ... on line N." /
"Fatal error: ..." message). Expectations for pun-tokens' ParseLexer and pun_format::lint.

usage: scripts/export-lint-cases.py   (needs php 8.5 on PATH)
"""
import json, os, subprocess
P = '<?php\n'
H = [  # heredoc / nowdoc
 "$x = <<<A\n\t  foo\n\t  A;\n", "$x = <<<A\n  foo\n\t  A;\n", "$x = <<<A\n\tfoo\n  A;\n", "$x = <<<A\n foo\n  A;\n",
 "$x = <<<A\n  foo\n\n  A;\n", "$x = <<<'A'\n foo\n  A;\n", "$x = <<<'A'\n\t foo\n\t A;\n", "$x = <<<A\n  {$a}\n  A;\n",
 "$x = <<<A\n  a{$a}\n b\n  A;\n", "$x = <<<A\n  a\n {$b}\n  A;\n", "$x = <<<A\n\t\tfoo\n\t\tA;\n", "$x = <<<A\n    foo\n\t\tA;\n",
 "$x = <<<A\n\t foo\n \tA;\n", "$x = <<<A\n  \t\n  A;\n", "$x = <<<A\n \n  A;\n", "$x = <<<A\n  foo\n  bar\n A;\n",
 "$x = <<<A\n  foo $a\n  A;\n", "$x = <<<A\nfoo\nA\n;\n", "$x = <<<A\n  A . 'x';\n", "$x = <<<A\n  foo\n  A\n", "$x = <<<A\nfoo", "$x = <<<'A'\nfoo", "$x = <<<A\nfoo\n", "$x = <<<A\n  foo\n  ", "$x = <<<A\n  {$a}\n\t", "$x = <<<A\n\tfoo\r\n", "$x = <<<A\n  x\\q\n ",
 "$x = <<<A\n  ${a}\n A;\n", "$x = <<<A\n  x\n\t\n  A;\n", "$x = <<<A\n\t\tx\n\t \n\t\tA;\n", "$x = <<<A\n  $a[1]\n A;\n",
 "$x = <<<A\n  x {$a} y\n   z\n  A;\n", "$x = <<<A\nA;\n", "$x = <<<  A\nfoo\nA;\n", '$x = <<<"A"\nfoo\nA;\n',
 "$x = <<<A\n   x\n  \tA;\n", "$x = <<<A\n\tx\n \tA;\n", "$x = <<<A\n\t \tx\n\t A;\n",
]
N = ["1__0;", "1_;", "0x;", "0b;", "0b2;", "08;", "0o8;", "0_1;", "1e;", "1e_1;", "1._0;", "0x_1;", "1_.0;", "0o;", "0O17;", "09.5;", "0777_;", "07_8;",
     "1_000.5_5;", "0x1_F;", "1e1_0;", "0b1_0;", "089;", "0_8;", "00;", "0o_7;", "1.e3;", ".5e_1;", "0.0_1;", "01e9;", "0xG;", "9223372036854775808;", "0b2_1;",
     "$a = 1__2;", "$a = 0_;", "07;", "0o777;", "0o_;", "078.5;", "0_8.5;", "08e1;"]
E = ['"\\u{}";', '"\\u{110000}";', '"\\u{zz}";', '"\\u{FFFFFF}";', '"\\u";', '"\\u{";', '"\\400";', '`\\u{zz}`;', "<<<A\n\\u{zz}\nA;\n", '"\\u{10FFFF}";', "'\\u{zz}';",
     '"$a \\u{xx}";', '"\\u{ 41}";', '"\\u{41";', '"\\u{0041}";', '"\\x";', '"\\xZ";', '"\\777";', "<<<'A'\n\\u{zz}\nA;\n", '"{$a}\\u{}";', 'b"\\u{zz}";']
HC = ["__halt_compiler", "__halt_compiler();", "__halt_compiler()", "__halt_compiler(1);", "function f(){ __halt_compiler(); }", "__halt_compiler;", "__halt_compiler() ?>x",
      "__HALT_COMPILER();junk(", "if (1) { __halt_compiler(); }", "namespace A { __halt_compiler(); }", "__halt_compiler();\n<?php ((", "__halt_compiler(); }"]
R = ["class list {}", "class int {}", "function list(){}", "const class = 1;", "class A { const class = 1; }", "namespace namespace;", "use static;", "$this = 1;", "function f(int $a, int $a){}",
     "break 0;", "break $x;", "continue 0;", "class A extends static {}", "interface A implements B {}", "readonly function f(){}", "abstract final class A {}", "class A { public public $x; }",
     "function f(): void {return 1;}", "list() = $a;", "$a?->b = 1;", "yield 1;", "match(1) {};", "match(1){default=>1,default=>2};", "isset(1);", "isset();", "new A(...);", "f(a: 1, 2);",
     "$a = &new A;", "const A = $x;", "declare(foo=1);", "namespace A; namespace B {}", "use A\\{};", "use A\\{B,};", "#[A] $x = 1;", "#[] function f(){}", "#[A,] function f(){}",
     "class A { public $a { get; } }", "interface I { public $a { get; } }", "function f(public $a){}", "$a |> f(...);", "(void) f();", "clone($a, ['x' => 1]);", "new A()->b();", "new A->b();",
     "class A { public private(set) int $a; }", "class A { private(set) int $a; }", "if (1): else if (2): endif;", "try {}", "function f(A&B|C $a){}", "function f((A&B)|C $a){}", "function f(?int|string $a){}",
     "function f(int|int $a){}", "function f(void $a){}", "1 ? 2 : 3 ? 4 : 5;", "(real) $x;", "(unset) $x;", "(integer) $x;", "class A { $a; }", "$a = [1, 2,,];", "[,] = $a;", "[$a,,$b] = $c;",
     "foreach ($a as &$k => $v) {}", "use A as B as C;", "enum A { case B = 1; }", "enum A: float {}", "fn($x) => { };", "match (1) { , };", "die 1;", "exit(1, 2);", "class A { abstract function f(){} }",
     "goto a;", "a: goto a;", "$a->{};", "\"{$a::B}\";", "static $a = f();", "function f() { static $a = f(); }", "global $a->b;", "unset($a,);", "isset($a,);", "f(,);", "function f($a,,){}",
     "new class {};", "new class extends {};", "class A { use T { a as protected; } }", "class A { use T { a as; } }", "class A { const int B = 1; }", "class A { final const B = 1; }", "interface I { final const B = 1; }",
     "class A { public function __construct(final public $a) {} }", "const A = static function () {};", "#[A(static fn () => 1)] function f(){}", "echo static::class;", "class A { const B = static::class; }",
     "namespace A\\class;", "namespace\\foo();", "\\namespace\\foo();", "A::class();", "$a::class;", "function f(never $a){}", "function &f(){}", "fn&($x) => $x;", "$$;", "${};", "\"$a[ 1]\";", "\"$a[-1]\";", "\"$a[01]\";",
     "\"$a[0x1]\";", "\"$a[-0]\";", "\"$a[1_0]\";", "\"$a[- 1]\";", "\"$a[-a]\";", "\"$a[1.5]\";", "\"$a->b->c\";", "\"$a?->b\";", "\"${a[1]}\";", "\"${a::b}\";", "`$a[-1]`;", "\"$a[99999999999999999999]\";", "\"$a[-9223372036854775808]\";",
     "class A { var public $a; }", "class A { public var $a; }", "class A { readonly $a; }", "class A { static static $a; }", "function f() { return static fn() => 1; }", "trait T { abstract private function f(); }",
     "switch ($a) { case 1; }", "switch ($a): default: default: endswitch;", "switch ($a) {;}", "catch (A) {}", "try {} catch (A) {}", "try {} catch (A|B $e) {}", "try {} catch () {}", "try {} finally {}",
     "list('a' => $a, $b) = $c;", "list($a, list()) = $b;", "[$a, []] = $b;", "[[]] = $a;", "list(&$a) = $b;", "[...$a] = $b;", "[$a, ...$b] = $c;", "f(...$a, ...$b);", "f(...$a, $b);", "f(a: 1, ...$b);",
     "function f(...$a, $b){}", "function f(...$a = []){}", "function f($a = 1, ...$b){}", "function f(int ...$a,){}", "$a = function () use ($a,) {};", "$a = function () use () {};", "$a = function () use ($this) {};",
     "$a = function () use (&$a) {};", "$a = fn() use ($a) => 1;", "class A { public function __get() {} }", "interface I { function f() {} }", "interface I { private function f(); }", "enum E { public $a; }",
     "enum E { case A; case A; }", "enum E: int { case A; }", "enum E { const A = 1; }", "enum E { use T; }", "enum E implements I {}", "enum E extends F {}", "enum E: int|string {}", "enum E: ?int {}",
     "new static;", "new static();", "new parent;", "new self;", "new class(1) {};", "instanceof A;", "$a instanceof 'A';", "$a instanceof A::B;", "$a instanceof (A);", "$a instanceof ($b);",
     "print print 1;", "echo 1, ;", "echo;", "include;", "require 'a', 'b';", "$a = 'unterminated", "$a = \"unterminated", "/* unterminated", "`unterminated", "?>", "$a = 1 ?>\n<?php ?>",
     "\x01;", "\x7f;", "$a\x80 = 1;", "\xef\xbb\xbf;", "$a = #comment\n1;", "#[A] fn() => 1;", "$a = #[A] fn() => 1;", "$a = #[A] static fn() => 1;", "#[A] class {}", "new #[A] class {};",
     "function f(#[A] $a){}", "#[A] enum E {}", "#[A] const X = 1;", "class A { #[A] const X = 1; }", "class A { #[A] use T; }", "class A { #[A] case X; }", "enum E { #[A] case X; }", "#[A] use B;",
     "$a = (int) (string) $b;", "$a = (bool)(float)(array)(object) $b;", "$a = (binary) $b;", "$a = (double) $b;", "$a = (boolean) $b;", "$a = ( int ) $b;", "$a = (\tint\t) $b;", "$a = (Int) $b;",
     "$a = $b ?: $c ?: $d;", "$a = $b ?? $c ?? $d;", "$a = 1 <=> 2 <=> 3;", "$a = 1 == 2 == 3;", "$a = 1 < 2 > 3;", "$a = !$b instanceof C;", "$a = -$b ** 2;", "$a = $b = $c = 1;", "$a = 1 + $b = 2;",
     "$a++ ++;", "++$a++;", "$a = &$b[];", "$a[] = &$b;", "$a = [&$b];", "$a = [...$b, ...$c];", "$a = ['a' => ...$b];", "$a = [...$b => 1];", "$a = $b[];", "f($a[]);", "$a = $b{1};", "$a{1} = 2;",
     "$a = \"$b{1}\";", "class A { function f() { $this->a::b(); } }", "A::$b::$c();", "A::{'b'}();", "$a->b::c;", "(clone $a)->b;", "clone $a->b;", "new $a->b;", "new $a::$b;", "new $a['b'];", "new ($a)();",
     "new (A::class);", "new A::$b;", "$a = (new A)::B;", "A::B::C;", "A::B();", "A::class::B;", "static::$a;", "parent::A;", "self::class;", "$a = 'a'::B;", "$a = 'a'::$b;", "$a = \"a\"::b();", "[1]::a();", "(1)::a();",
     "__LINE__ = 1;", "__CLASS__::a();", "$a = __CLASS__::a;", "exit;", "exit();", "die();", "exit(1);", "exit(...);", "die(...);", "$a = exit;", "f(exit);", "throw;", "$a = throw $b;", "$a ?? throw $b;",
     "fn() => throw $a;", "$a = yield;", "function f() { $a = yield; }", "function f() { yield $a => $b; }", "function f() { $a = yield $b => $c; }", "function f() { yield from; }", "function f() { yield from $a; }",
     "function f() { $a = (yield); }", "function f() { f(yield $a); }", "function f() { f(yield $a, 1); }", "function f() { $a = yield + 1; }", "function f() { return yield; }", "function f() { yield yield; }",
     "declare(strict_types=1) {}", "declare(ticks=1): enddeclare;", "declare(strict_types=1, ticks=1);", "declare(strict_types=2);", "declare(encoding='UTF-8');", "declare();", "declare(strict_types);",
     "goto 1;", "a: a: ;", "class A { a: }", "function f() { a: }", "namespace A; namespace B;", "namespace A {} namespace B {}", "namespace {} namespace {}", "namespace A {} echo 1;", "echo 1; namespace A;",
     "namespace A; use B; use C\\{D, E as F, function g, const H};", "use function A\\{b, c};", "use A\\{function b, const c};", "use function A\\{function b};", "use A\\B\\{C\\D};", "use \\A;", "use \\A\\{B};", "use A\\{\\B};",
     "use A as list;", "use A as class;", "use A as int;", "use A as self;", "use A as readonly;", "use A as enum;", "use A as mixed;", "use function A as list;", "class readonly {}", "class enum {}", "class mixed {}", "class never {}",
     "class A extends readonly {}", "function readonly() {}", "readonly();", "enum();", "enum::A;", "class A { const enum = 1; }", "class A { function list() {} }", "class A { function class() {} }", "A::list();", "A::class;",
     "$a->class;", "$a->list();", "$a::list;", "class A { const function = 1; }", "A::function;", "class A { public function __halt_compiler() {} }", "A::__halt_compiler();", "$a->__halt_compiler();", "function __halt_compiler() {}",
     "class A { use B { list as foo; } }", "class A { use B { foo as list; } }", "class A { use B { foo as public list; } }", "class A { use B { foo as class; } }", "class A { use B { B::foo insteadof C; } }", "class A { use B { foo insteadof C; } }",
     "class A { use B { B::foo as; } }", "class A { use B, C {} }", "class A { use B, C; }", "class A { use; }", "interface I extends A, B {}", "interface I extends A, {}", "class A implements B, {}", "class A extends B, C {}",
     "class A { public function f(): static {} }", "function f(): static {}", "function f(static $a) {}", "function f(): ?static {}", "function f(): null {}", "function f(): false {}", "function f(): true {}", "function f(): ?mixed {}",
     "function f(): ?null {}", "function f(): void|null {}", "function f(): (A&B)|null {}", "function f(): A&B {}", "function f(): (A|B)&C {}", "function f(): (A&B) {}", "function f(): ((A&B)|C) {}", "function f(): ?A&B {}", "function f(): ?(A&B) {}",
     "function f(&...$a) {}", "function f(...&$a) {}", "function f(int &...$a) {}", "function f(&$a = 1) {}", "f(&$a);", "f(... $a);", "function f(?...$a) {}", "function f(int|...$a) {}",
     "class A { public function __construct(public $a, public ...$b) {} }", "class A { public function __construct(public readonly $a) {} }", "class A { public function __construct(readonly $a) {} }", "class A { public function __construct(public public $a) {} }",
     "class A { public function __construct(protected(set) $a) {} }", "class A { public function __construct(public $a { get => 1; }) {} }", "class A { public $a { set($v) { } } }", "class A { public $a { get; set; } }", "class A { public $a { } }",
     "class A { public $a { foo; } }", "class A { public $a { final get; } }", "class A { public $a { &get; } }", "class A { public $a { get => 1 } }", "class A { public $a { get { return 1; } } }", "class A { public $a { set(int $v,) {} } }",
     "class A { public $a { set($v = 1) {} } }", "class A { public $a { get(); } }", "class A { public $a, $b { get; } }", "class A { abstract public $a { get; } }", "class A { public $a = 1 { get; } }", "class A { public $a { #[A] get; } }",
     "class A { public $a { get => 1; get => 2; } }", "class A { public $a { private get; } }", "class A { public $a { static get; } }", "class A { public readonly $a { get; } }", "class A { var $a { get; } }", "class A { public static $a { get; } }",
     "class A { public final $a; }", "class A { final $a; }", "class A { public abstract $a; }", "class A { abstract $a { get; } }", "class A { final public const A = 1; }", "class A { private final const A = 1; }",
     "class A { public function f(); }", "class A { abstract public function f() {} }", "class A { abstract public function f(); }", "class A { static function f(); }", "class A { function f() }", "class A { public function f() {}; }",
     "class A { ; }", "class A {} ;", "function f() {};", "class A { public $a = 1, $b = 2; }", "class A { public $a, $b; }", "class A { public $a, ; }", "class A { const A = 1, B = 2; }", "class A { const A, B; }", "class A { const A; }",
     "class A { public int $a; }", "class A { public ?int $a; }", "class A { public int|string $a; }", "class A { public void $a; }", "class A { public callable $a; }", "class A { public never $a; }", "class A { public static $a; }",
     "class A { public static static $a; }", "class A { public function public() {} }", "class A { public function static() {} }", "class A { public function new() {} }", "class A { public function self() {} }", "class A { public function parent() {} }",
     "class A { const self = 1; }", "class A { const parent = 1; }", "class A { const static = 1; }", "class A { const new = 1; }", "A::new();", "A::static();", "$a->static;", "A::self;", "A::new;", "A::self();",
     "class A { public function f() { return new static::$b; } }", "new static::$a;", "new self::$a;", "new A::B;", "new A::B();", "new $a::B;", "new ('A');", "new (f());", "new A()['b'];", "new A()::B;", "new A()::$b;", "new A()->b;", "new A()?->b;", "new A->b;",
     "new A()();", "new class {}->a();", "new class {}::A;", "new A() instanceof B;", "new A()++;", "clone new A()->b;", "(new A)->b;", "new A ?? 1;", "new A()->b = 1;", "new A()::$b = 1;", "new A()['b'] = 1;", "new A ->b;", "new A\n->b();",
     "$a = new A() ['b'];", "$a = new A() -> b();", "$a = new readonly class {};", "$a = new final class {};", "$a = new abstract class {};", "$a = new #[A] readonly class {};", "$a = new readonly #[A] class {};",
     "readonly class A {}", "final readonly class A {}", "readonly final class A {}", "abstract readonly class A {}", "readonly abstract class A {}", "readonly readonly class A {}", "final final class A {}", "readonly interface I {}", "readonly trait T {}", "readonly enum E {}", "final enum E {}", "abstract enum E {}", "final trait T {}", "final interface I {}",
     "readonly function f() {}", "readonly fn() => 1;", "readonly $a;", "readonly class {}",
     "$a = <<<A\nA . <<<B\nB;\n", "$a = [<<<A\n  x\n  A, 1];\n", "$a = f(<<<A\n  x\n  A);\n", "$a = <<<A\n  x\n  A\n  ;\n", "$a = <<<A\n  x\n  AB;\n", "$a = <<<A\n  x\n  A_;\n", "$a = <<<A\n  x\n  A1;\n", "$a = <<<A\n  x\n  A;B;\n",
     "$a = <<<A\nAA\nA;\n", "$a = <<<A\n  AA\n  A;\n", "$a = <<<A\n  A\n  A;\n", "$a = <<<A\n x\n  A::B;\n", "$a = <<<A\n   x\n  A->b;\n",
]
C = [P + s for s in H + N + E + R] + ["<?php " + s for s in HC] + ["#!/usr/bin/env php\n<?php 1;", "<?= 1 ?>", "<?=;", "x<?php ", "<?php", "<?php\n", "<?phpx", "<?PHP 1;", "<?php\r1;", "<?php\t1;", "<?php ?>\n<?php }", "<?php if (1): ?>x<?php else: ?>y<?php endif ?>", "<?php if (1): ?>x<?php else if (2): ?>y<?php endif ?>", "<?php if (1) { ?>x<?php } else ?>y", "<?php if (1) ?>x", "<?php switch ($a): ?>\n<?php case 1: endswitch;", "<?php switch ($a): ?>  <?php case 1: endswitch;", "<?php switch ($a): ?>x<?php case 1: endswitch;", "<?php switch ($a) { ?>\n<?php case 1: }", "<?php switch ($a) { ?>x<?php case 1: }"]

# Lines, CRLF, close tags, heredoc/nowdoc details, scan-ahead, modifiers, encoding, `<?=`, expected-token lists.
X = [
 "<?php\r\n$a = 1\r\n\r\n}", "<?php\r$a = 1\r\r}", "<?php\n$a = 1 ?>\n\n<?php }", "<?php\n$a = 'x\ny' 1;", "<?php\n$a = \"x\ny\" 1;", "<?php\n$a = \"a\n\\u{zz}\";",
 "<?php\n$a = \"a\n$b\n\\u{zz}\";", "<?php\n$a = `a\n\\u{110000}`;", "<?php\n$a = \"\\u{0000000041}\";", "<?php\n$a = \"\\u{00000000110000}\";", "<?php\n$a = \"\\\\u{zz}\";",
 "<?php\n$a = <<<A\n  x\n  \\u{zz}\n  A;\n", "<?php\n$a = <<<A\n  x\n  \\u{zz}\n A;\n", "<?php\n$a = <<<A\n  x $b\n  \\u{zz}\n  A;\n",
 "<?php\n$a = <<<A\n{$b}\n  A;\n", "<?php\n$a = <<<A\n${b}\n  A;\n", "<?php\n$a = <<<A\n$b\n\t\tA;\n", "<?php\n$a = <<<A\n$b\n  x\n  A;\n",
 "<?php\n$a = <<<A\n{$b}\n  y\n  {$c}\n z\n A;\n", "<?php\n$a = <<<A\n{$b(<<<B\n  x\n  B)}\n  A;\n", "<?php\n$a = <<<A\n{$b(<<<B\n  x\n \tB)}\n  A;\n",
 "<?php\n$a = <<<A\n{$b(<<<B\n\t B)}\n  A;\n", "<?php\n$a = <<<A\n{$b[08]}\n  A;\n", "<?php\n$a = <<<A\n{$b[\"\\u{zz}\"]}\n  A;\n", "<?php\n$a = <<<A\n{$b /* x\n  A;\n",
 "<?php\n$a = <<<A\n{$b)}\n  A;\n", "<?php\n$a = <<<A\n{$b(]}\n  A;\n", "<?php\n$a = <<<A\n{$b(}\n  A;\n", "<?php\n$a = <<<A\n{$b[`x`]}\n  A;\n", "<?php\n$a = <<<A\n{$b ?>\n<?php }\n  A;\n",
 "<?php\n$a = <<<A\n{$b}\n  x\n\t  A;\n", "<?php\n$a = <<<A\n{$b} \n", "<?php\n$a = <<<A\n  x\n  ", "<?php\n$a = <<<A\n  x\n y", "<?php\n$a = <<<A\n  x\n\t", "<?php\n$a = <<<'A'\n  x\n y",
 "<?php\n$a = <<<'A'\n  x\n\t  A;\n", "<?php\n$a = <<<'A'\n x\n\n  A;\n", "<?php\n$a = <<<A\r\n  x\r\n y\r\n  A;\r\n", "<?php\n$a = <<<A\r\n  x\r\n  A . 1 1;\r\n", "<?php\n$a = <<<A\n  x\n  A . 1 1;\n",
 "<?php\n$a = <<<A\n  $x\n  A . 1 1;\n", "<?php\n$a = <<<'A'\n  x\n  A . 1 1;\n", "<?php\n$a = <<<A\nA . 1 1;\n", "<?php\n$a = <<<A\n\nA . 1 1;\n", "<?php\n$a = <<<A\n  a\n\n  A . 1 1;\n",
 "<?php\nclass A { final abstract function f(); }", "<?php\nclass A { abstract final $a { get; } }", "<?php\nclass A { public function __construct(public private(set) protected(set) $a) {} }", "<?php\nclass A { public private(set) private(set) $a; }",
 "<?php\nclass A { readonly function f() {} }", "<?php\nclass A { abstract const X = 1; }", "<?php\nclass A { static const X = 1; }", "<?php\nclass A { public function __construct(abstract $a) {} }", "<?php\nclass A { public $a { final final get; } }",
 "<?php\nclass A { public $a { abstract get; } }", "<?php\nclass A { public $a { readonly get; } }", "<?php\nclass A { public $a { public(set) get; } }", "<?php\nclass A { use T { f as readonly; } }", "<?php\nclass A { use T { f as readonly g; } }",
 "<?php\nclass A { use T { f as public(set); } }", "<?php\nclass A { use T { f as protected g; } }", "<?php\nclass A { use T { f as static; } }", "<?php\nclass A { protected(set) function f() {} }", "<?php\nclass A { private(set) const X = 1; }",
 "<?php\nclass A { public(set) static $a; }", "<?php\nabstract abstract class A {}", "<?php\nfinal abstract class A {}", "<?php\n$a = new readonly readonly class {};", "<?php\n$a = new readonly final class {};",
 "<?php\nclass A { public static function f() {} public static $a; private const X = 1; }", "<?php\nclass A { private ?int $a; protected static ?A $b = null; }",
 "<?php\ndeclare(encoding='UTF-8', ticks=1);", "<?php\ndeclare(ticks=1, encoding=FOO);", "<?php\ndeclare(encoding=(('a')));", "<?php\ndeclare(encoding='a' . ('b' . 1));", "<?php\ndeclare(encoding='a' . f());",
 "<?php\ndeclare(encoding=<<<A\nx $b\nA);", "<?php\ndeclare(encoding=[1, 2]);", "<?php\ndeclare(encoding=f(1, 2));",
 "<?php class A { const ?><?= = 1; }", "<?php class A { function ?><?= () {} }", "<?php class A { use T { f as ?><?= ; } }", "<?php A::?><?= ();", "<?php class A { const ECHO = 1; } A::echo();",
 "<?php\n$a = \\;", "<?php\n$a = &;", "<?php\n$a = \"x\" \"y\";", "<?php\n$a = 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx' 1;", "<?php\n$a = 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx' 1;",
 "<?php\n$a = 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx' 1;", "<?php\n$a = 1 \x01;", "<?php\n$a = 1 \"\";", "<?php\n$a = \"$b\" \"$c\";", "<?php\n$a = \"x$b\";\"", "<?php\nfunction f(int $a) { return $a }",
 "<?php\nif ($a) { } else", "<?php\nforeach ($a as $b) echo", "<?php\n$a->", "<?php\n$a::", "<?php\nnew", "<?php\nuse", "<?php\nnamespace", "<?php\nclass A extends", "<?php\n#[", "<?php\n$a = [1, 2", "<?php\n$a = (1", "<?php\n{",
 "<?php\n$a = 1\n?>", "<?php\necho 1 ?>x<?php echo 2", "x<?php echo 1", "<?php\n$a = \"\\400\";", "<?php\n$x = 0b_1;", "<?php\n$x = 0_0;", "<?php\n$x = 0089.5;", "<?php\n$x = 0_8_9e1;",
 "<?php\n$a = (REAL) $b;", "<?php\n$a = ( Real\t) $b;", "<?php\n$a = (double) $b . (float) $c;", "<?php\n/** doc */\nclass A {}\n/* x */ /** y", "<?php\n// x\n# y\n/*\n*/ }", "<?php\n$a = \"${a}\" . \"{$b->c}\";",
 "<?php\n$a = \"{$b}\" . `{$c}` . `${d}`;", "<?php\n$a = \"$a[1]$b->c$d?->e\";", "<?php\n$a = b<<<A\n  x\n A;\n", "<?php\n$a = B'x' . b\"y\\u{zz}\";", "<?php\n$a = b'\\u{zz}' . '\"';",
 "<?php\nfn() => yield $a;", "<?php\n$f = fn&(array &$a): array => $a;", "<?php\n$a = $b |> strlen(...) |> f(...);", "<?php\nenum E: string implements I { case A = 'a'; const B = self::A; }", "<?php\n__halt_compiler();\n\x00\xff",
 "<?php\n__halt_compiler() ?>junk", "<?php\n(\n]", "<?php\n{\n)", "<?php\n$a = 'x\ny", "<?php\nclass A { public $a { protected get; } }", "<?php\nclass A { public $a { private get; } }", "<?php\nclass A { public $a { public get; } }",
 "<?php\n$a = 1 'x\ny';", "<?php\n$a = 1 b'x';", "<?php\n$a = 1 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx';", "<?php\n$a = 1 \"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\";", "<?php\n$a = 1 \"$x\";", "<?php\n__halt_compiler()\n;", "<?php\n__halt_compiler(); (", "<?php\n{ __halt_compiler(); }",
]
C += X


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    php = r'''$out = [];
foreach (json_decode(stream_get_contents(STDIN)) as $code) {
    try { token_get_all($code, TOKEN_PARSE); $e = null; }
    catch (\CompileError $x) { $e = ($x instanceof \ParseError ? 'Parse error' : 'Fatal error').': '.$x->getMessage().' on line '.$x->getLine().'.'; }
    $out[] = ['code' => $code, 'error' => $e];
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_INVALID_UTF8_SUBSTITUTE), "\n";'''
    r = subprocess.run(['php', '-d', 'display_errors=stderr', '-r', php], input=json.dumps(C), capture_output=True, text=True, check=True)
    path = os.path.join(root, 'crates/pun-tokens/tests/fixtures/lint_cases.json')
    open(path, 'w').write(r.stdout)
    print(f'wrote {path}: {len(C)} cases')


main()
