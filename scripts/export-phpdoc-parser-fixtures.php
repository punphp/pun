<?php

declare(strict_types=1);

/*
 * Oracle fixtures for pun_phpstan::phpdoc (phpstan/phpdoc-parser as PHPStan 2.3.1 bundles it): the tags
 * PhpDocParser reads from doc comments, with the ParserException messages of invalid values, and the result of
 * TypeStringResolver's parse step on type strings.
 *
 *   php scripts/export-phpdoc-parser-fixtures.php > crates/pun-phpstan/tests/fixtures/phpdoc-tags.json
 *
 * The doc comments are the hand-written ones below, every doc comment of PHPStan's PhpDoc and Classes rule test
 * data, and the doc comments with an invalid tag value anywhere in PHPStan's test data.
 */

$root = dirname(__DIR__);
require $root.'/.oracle/phpstan-phar/vendor/autoload.php';

use PHPStan\PhpDocParser\Ast\PhpDoc\InvalidTagValueNode;
use PHPStan\PhpDocParser\Ast\PhpDoc\TypeAliasTagValueNode;
use PHPStan\PhpDocParser\Ast\Type\InvalidTypeNode;
use PHPStan\PhpDocParser\Lexer\Lexer;
use PHPStan\PhpDocParser\Parser\ConstExprParser;
use PHPStan\PhpDocParser\Parser\ParserException;
use PHPStan\PhpDocParser\Parser\PhpDocParser;
use PHPStan\PhpDocParser\Parser\TokenIterator;
use PHPStan\PhpDocParser\Parser\TypeParser;
use PHPStan\PhpDocParser\ParserConfig;

$config = new ParserConfig(['lines' => true]);
$lexer = new Lexer($config);
$constExprParser = new ConstExprParser($config);
$typeParser = new TypeParser($config, $constExprParser);
$phpDocParser = new PhpDocParser($config, $typeParser, $constExprParser);

/** @return list<array{string, int, ?string, ?string}>|null */
function tags(string $doc): ?array
{
    global $lexer, $phpDocParser;
    try {
        $node = $phpDocParser->parse(new TokenIterator($lexer->tokenize($doc)));
    } catch (ParserException) {
        return null;
    }
    $tags = [];
    foreach ($node->getTags() as $tag) {
        $value = null;
        $message = null;
        if ($tag->value instanceof InvalidTagValueNode) {
            $value = $tag->value->value;
            $message = $tag->value->exception->getMessage();
        } elseif ($tag->value instanceof TypeAliasTagValueNode && $tag->value->type instanceof InvalidTypeNode) {
            $message = $tag->value->type->getException()->getMessage();
        }
        $tags[] = [$tag->name, $tag->getAttribute('startLine'), $value, $message];
    }

    return $tags;
}

$docs = [
    "/** @var int */",
    "/** */",
    "/**\n * text\n *\n * more text\n */",
    "/**\r\n * @param int \$a text\r\n *   continued\r\n * @return void\r\n */",
    "/** @param int \$a */",
    "/** @param \$a */",
    "/** @param &...\$a */",
    "/** @param int &...\$a desc */",
    "/** @param int */",
    "/** @param int| \$a */",
    "/** @param-immediately-invoked-callable \$a */",
    "/** @param-later-invoked-callable a */",
    "/** @phpstan-pure-unless-callable-is-impure \$f */",
    "/** @pure-unless-parameter-passed \$f */",
    "/** @param-closure-this Foo \$a */",
    "/** @param-closure-this \$a */",
    "/** @var int \$a desc */",
    "/** @var \$this */",
    "/** @var int \$this */",
    "/** @var int|string|null */",
    "/** @var int | */",
    "/** @var int&Foo */",
    "/** @var int|Foo&Bar */",
    "/** @var (int|string)[] */",
    "/** @var int[][] */",
    "/** @var int [] */",
    "/** @var Foo[Bar] */",
    "/** @var Foo[Bar */",
    "/** @var \$this[] */",
    "/** @var array{} */",
    "/** @var array{int, string} */",
    "/** @var array{a: int, 'b'?: string, \"c\": float, 0: bool, Foo::BAR: int} */",
    "/** @var array{a: int, ...} */",
    "/** @var array{a: int, ...<string>} */",
    "/** @var array{a: int, ...<int, string>} */",
    "/** @var list{int, ...<string>} */",
    "/** @var list{int, ...<int, string>} */",
    "/** @var array{a: int,} */",
    "/** @var array{a: int */",
    "/** @var array{Foo::: int} */",
    "/** @var array{\n *   a: int, // comment\n *   b: string,\n * } */",
    "/** @var array {a: int} */",
    "/** @var array{a: int}[] */",
    "/** @var object{a: int, 'b'?: string, \"c\": int} */",
    "/** @var object{} */",
    "/** @var object{a: int}[] */",
    "/** @var object{a} */",
    "/** @var object{1: int} */",
    "/** @var object{a: int */",
    "/** @var list<int> */",
    "/** @var array<int, string> */",
    "/** @var array<int, string,> */",
    "/** @var array<int, string>[] */",
    "/** @var array<*, int> */",
    "/** @var Foo<covariant int, contravariant string> */",
    "/** @var array<int */",
    "/** @var array< */",
    "/** @var array<int, > */",
    "/** @var Foo<\n *   int,\n *   string\n * > */",
    "/** @var callable(int, string): void */",
    "/** @var callable(int &\$a, string ...\$b, int \$c=): void */",
    "/** @var callable(): ?int */",
    "/** @var callable(): (int|string) */",
    "/** @var callable(): (int|string)[] */",
    "/** @var callable(): \$this */",
    "/** @var callable(): \$this[] */",
    "/** @var callable(): array<int> */",
    "/** @var callable(): array<int>[] */",
    "/** @var callable(): int[] */",
    "/** @var callable(): array{a: int} */",
    "/** @var callable(): array{a: int}[] */",
    "/** @var callable(): object{a: int} */",
    "/** @var callable(): Foo::BAR */",
    "/** @var callable(): 1 */",
    "/** @var callable(): 'a'[] */",
    "/** @var callable(): */",
    "/** @var callable(): [] */",
    "/** @var callable(int, ): void */",
    "/** @var callable(\n *   int,\n * ): void */",
    "/** @var callable(int */",
    "/** @var callable<T>(T): T */",
    "/** @var callable<T of int, U = string,>(T): T */",
    "/** @var Closure<T>(T): T[] */",
    "/** @var Closure<T */",
    "/** @var callable(int)[] */",
    "/** @var \\Closure(int): void */",
    "/** @var <a> text </a> */",
    "/** @var array<a> text </a> */",
    "/** @var array<a> text */",
    "/** @var array<a>b</a> */",
    "/** @var array<a */",
    "/** @var array<a> */",
    "/** @var ?int */",
    "/** @var ? */",
    "/** @var 1 */",
    "/** @var 1.5 */",
    "/** @var 1_000 */",
    "/** @var 'foo' */",
    "/** @var \"foo\\n\" */",
    "/** @var 'a'[] */",
    "/** @var Foo::BAR */",
    "/** @var Foo::BAR_* */",
    "/** @var Foo::* */",
    "/** @var Foo::*BAR */",
    "/** @var Foo::* BAR */",
    "/** @var Foo:: */",
    "/** @var Foo::BAR[] */",
    "/** @var null::FOO */",
    "/** @var true::FOO */",
    "/** @var false::FOO */",
    "/** @var array::FOO */",
    "/** @var [1, 2] */",
    "/** @var \$this */",
    "/** @var (int) */",
    "/** @var (int */",
    "/** @var (\n *   int\n *   | string\n * ) */",
    "/** @var (\n *   int\n *   & Foo\n * ) */",
    "/** @var int\n *   | string */",
    "/** @var int\n *   & Foo */",
    "/** @var (T is int ? string : bool) */",
    "/** @var (T is not int ? string : bool) */",
    "/** @var (\$a is int ? string : bool) */",
    "/** @var (\$a is not int ? string : bool) */",
    "/** @var (\$a isnt int ? string : bool) */",
    "/** @var (T is int ? string) */",
    "/** @var (T is int string) */",
    "/** @return (T is int ? string : (T is string ? int : bool)) */",
    "/** @return T is int ? string : bool */",
    "/** @return int desc */",
    "/** @return int|desc */",
    "/** @return int| desc */",
    "/** @return int& desc */",
    "/** @return int[desc */",
    "/** @return array<int>desc */",
    "/** @return */",
    "/** @return !int */",
    "/** @throws Foo */",
    "/** @mixin Foo */",
    "/** @phpstan-require-extends Foo */",
    "/** @phpstan-require-implements Foo */",
    "/** @phpstan-sealed Foo|Bar */",
    "/** @psalm-inheritors Foo */",
    "/** @phpstan-self-out self<int> */",
    "/** @phpstan-this-out */",
    "/** @deprecated */",
    "/** @deprecated since 1.0\n *   use Foo */",
    "/** @property int \$a */",
    "/** @property-read int */",
    "/** @property-write \$a */",
    "/** @method void foo() */",
    "/** @method static void foo(int \$a, string &\$b, ...\$c) */",
    "/** @method static foo() */",
    "/** @method foo() */",
    "/** @method static static foo() */",
    "/** @method int foo(\$a = 1, \$b = 'a', \$c = null, \$d = [], \$e = array(1 => 2), \$f = Foo::BAR, \$g = true, \$h = false, \$i = 1.5, \$j = FOO) */",
    "/** @method int foo(\$a = [1, 2,], \$b = array()) */",
    "/** @method int foo(\$a = [1 */",
    "/** @method int foo(\$a = array( */",
    "/** @method int foo(\$a = ) */",
    "/** @method int foo(?int \$a, (int|string) \$b) */",
    "/** @method T foo<T, U of int>(T \$a) */",
    "/** @method T foo<T>( */",
    "/** @method int[] */",
    "/** @method int */",
    "/** @method int foo */",
    "/** @method int foo(int) */",
    "/** @method int foo(\$a, ) */",
    "/** @template T */",
    "/** @template T of int */",
    "/** @template T as int */",
    "/** @template T super int */",
    "/** @template T = int */",
    "/** @template T of int = 1 desc */",
    "/** @template-covariant T */",
    "/** @template */",
    "/** @template T of */",
    "/** @extends Foo<int> */",
    "/** @extends Foo<int> desc */",
    "/** @extends Foo<int>desc */",
    "/** @implements Foo */",
    "/** @use Foo<int> */",
    "/** @extends */",
    "/** @phpstan-type Foo int */",
    "/** @phpstan-type Foo = int */",
    "/** @phpstan-type Foo int|string */",
    "/** @phpstan-type Foo array{\n *   a: int,\n * }\n */",
    "/** @phpstan-type Foo int desc */",
    "/** @phpstan-type Foo array< */",
    "/** @phpstan-type */",
    "/** @phpstan-type 1 */",
    "/** @phpstan-import-type Foo from Bar */",
    "/** @phpstan-import-type Foo from Bar as Baz */",
    "/** @phpstan-import-type Foo from Bar extra */",
    "/** @phpstan-import-type Foo from Bar\n * @var int */",
    "/** @phpstan-import-type Foo */",
    "/** @phpstan-import-type Foo to Bar */",
    "/** @phpstan-import-type Foo from */",
    "/** @phpstan-import-type Foo from Bar as */",
    "/** @phpstan-assert int \$a */",
    "/** @phpstan-assert !int \$a */",
    "/** @phpstan-assert =int \$a */",
    "/** @phpstan-assert !=int \$a */",
    "/** @phpstan-assert int \$this */",
    "/** @phpstan-assert int \$this->a */",
    "/** @phpstan-assert int \$a->b() */",
    "/** @phpstan-assert int \$a->b( */",
    "/** @phpstan-assert int \$a-> */",
    "/** @phpstan-assert int */",
    "/** @phpstan-assert-if-true int \$a */",
    "/** @param-out int \$a */",
    "/** @param-out int */",
    "/** @phpstan-ignore-next-line */",
    "/** @foo */",
    "/** @foo bar\n * baz\n *\n * qux */",
    "/** @foo bar @baz */",
    "/** @foo bar\n * @baz qux */",
    "/** @foo\n * @var int */",
    "/** @foo text @var int */",
    "/** @foo text@var int */",
    "/** @foo text @var */",
    "/** @foo text @bar(1) */",
    "/** @foo text @var( */",
    "/** @foo @var \$x( */",
    "/** @Foo() */",
    "/** @Foo(1, \"a\", 'b', true, null, Foo::BAR, {1, 2}, @Bar(), a=1, b={c: 1, \"d\"=2, 3=4}) */",
    "/** @Foo(1, \"a\", true, null, Foo::BAR, {1, 2}, @Bar(), @Baz, a=1, b={c: 1, \"d\"=2, 3=4, 'e'=5}) */",
    "/** @Foo({}) */",
    "/** @Foo(\"a\n * b\") */",
    "/** @Foo(\"a\"\"b\n * c\") */",
    "/** @Foo({\"a\n * b\"=1}) */",
    "/** @Foo({Foo::BAR=1}) */",
    "/** @Foo({Foo::=1}) */",
    "/** @Foo(Foo::) */",
    "/** text \x0b */",
    "/** @foo text @_Bar */",
    "/** @foo text \x0b */",
    "/** @Foo(\n *   a=1,\n *   b=2\n * ) */",
    "/** @Foo({a=1, Foo::BAR=2, \"x\" \"y\"=3}) */",
    "/** @Foo({\"x\"\"y\"=3}) */",
    "/** @Foo({null::X=1}) */",
    "/** @Foo({=1}) */",
    "/** @Foo({1 2}) */",
    "/** @Foo(a b) */",
    "/** @Foo([1]) */",
    "/** @Foo('a') */",
    "/** @Foo(\"a\"\"b\") */",
    "/** @Foo(\"a\" \"b\") */",
    "/** @Foo(a=@Bar(1), b=Foo::BAR) */",
    "/** @Foo(1 desc */",
    "/** @Foo(1) desc\n * more */",
    "/** @Foo(1) @var int */",
    "/** @Foo(1)\n * @var int */",
    "/** @_Foo(1) */",
    "/** @_Foo(1 */",
    "/** @_Foo text */",
    "/** @\\Foo\\Bar(1) */",
    "/** @var int @phpstan-ignore-line */",
    "/** @var int\n */",
    "/**\n * @var int\n *\n * @return void\n */",
    "/** @var int // comment */",
    "/** @var array{\n * // comment\n * a: int\n * } */",
    "/** @var array{ // c\n * a: int, // c2\n * } */",
    "/** @var (\n * // comment\n * int\n * ) */",
    "/** @var array{int\n * , string} */",
    "/** @var array{...<int\n * , string\n * >} */",
    "/** @var Foo<\n * // c\n * int> */",
    "/** @var \$a is int ? string : bool */",
    "/** @var \$a */",
    "/** @var \\Foo\\Bar */",
    "/** @var non-empty-array<int> */",
    "/** @var non-empty-list{int} */",
    "/** @var int-mask-of<Foo::*> */",
    "/** @var key-of<Foo::BAR> */",
    "/** @phpstan-var int */",
    "/** @phpstan-param int \$a */",
    "/** @phpstan-return int */",
    "/** @phpstan-unknown foo */",
    "/** @psalm-param int */",
    "/** @phan-var int */",
    "/** @var int\r */",
    "/** @var int \x0b */",
    "/**\n\t* @var int\n\t*/",
    "/** @var int\n * text */",
    "/** @var Foo <int> */",
    "/** @return Foo<int */",
];

$files = [];
foreach (['Rules/PhpDoc/data', 'Rules/Classes/data'] as $dir) {
    foreach (glob($root.'/.oracle/phpstan-src/tests/PHPStan/'.$dir.'/*.php') as $f) {
        $files[$f] = true;
    }
}
$interesting = [];
$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root.'/.oracle/phpstan-src/tests/PHPStan'));
foreach ($it as $f) {
    if ($f->getExtension() !== 'php') {
        continue;
    }
    $all = isset($files[$f->getPathname()]);
    foreach (@token_get_all((string) file_get_contents($f->getPathname())) as $t) {
        if (! is_array($t) || $t[0] !== T_DOC_COMMENT) {
            continue;
        }
        if ($all) {
            $docs[] = $t[1];
            continue;
        }
        $tags = tags($t[1]);
        if ($tags === null || array_filter($tags, fn ($tag) => $tag[3] !== null) !== []) {
            $docs[] = $t[1];
        }
    }
}
$docs = array_values(array_unique($docs));

$types = [
    'int', 'int|string', '?int', 'array<int, string>', 'int[]', 'Foo[Bar]', '(int|string)', 'int|', 'array<',
    '*', 'int|*', 'callable(int): void', 'callable(int', 'Foo::BAR', '[1]', "int\n", '1 2', 'int is', '$a',
];

$out = ['docs' => [], 'types' => []];
foreach ($docs as $doc) {
    $out['docs'][] = ['doc' => $doc, 'tags' => tags($doc)];
}
foreach ($types as $type) {
    $tokens = new TokenIterator($lexer->tokenize($type));
    try {
        $typeParser->parse($tokens);
        $tokens->consumeTokenType(Lexer::TOKEN_END);
        $error = null;
    } catch (ParserException $e) {
        $error = $e->getMessage();
    }
    $out['types'][] = ['type' => $type, 'error' => $error];
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_INVALID_UTF8_SUBSTITUTE | JSON_THROW_ON_ERROR)."\n";
