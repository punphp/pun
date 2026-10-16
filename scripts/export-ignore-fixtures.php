<?php

declare(strict_types=1);

/*
 * Exact results of PHPStan 2.2.7's ignore logic, from the phar's own classes:
 *
 *   tests/fixtures/comments.json  RichParser::getLinesToIgnore() over php-parser's tokens (the inputs of
 *                                 phpstan-src's RichParserTest and IgnoreLexerTest, plus extra cases)
 *   tests/fixtures/local.json is scripts/export-local-ignore-fixtures.php's; this one also writes
 *   tests/fixtures/local-processor.json  LocalIgnoresProcessor::process() on getLinesToIgnore() of a code sample and
 *                                 hand-picked errors (with/without identifier, non-ignorable, other file)
 *   tests/fixtures/ignore.json    IgnoredErrorHelper::initialize() + IgnoredErrorHelperResult::process() over
 *                                 ignoreErrors entries and errors (paths under tests/fixtures/ignore/files, as {dir})
 *
 *   php scripts/export-ignore-fixtures.php
 */

namespace PHPStan\Testing {
    abstract class PHPStanTestCase {}
}

namespace {
    use PHPStan\Analyser\Error;
    use PHPStan\Analyser\Ignore\IgnoredErrorHelper;
    use PHPStan\Analyser\Ignore\IgnoreLexer;
    use PHPStan\File\FileHelper;
    use PHPStan\Parser\RichParser;

    $root = dirname(__DIR__);
    $fixtures = $root.'/crates/pun-phpstan/tests/fixtures';
    Phar::loadPhar($root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar', 'phpstan.phar');
    require 'phar://phpstan.phar/vendor/autoload.php';
    require $root.'/.oracle/phpstan-src/tests/PHPStan/Parser/RichParserTest.php';
    require $root.'/.oracle/phpstan-src/tests/PHPStan/Analyser/Ignore/IgnoreLexerTest.php';

    function write(string $file, mixed $data): void
    {
        file_put_contents($file, json_encode($data, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_INVALID_UTF8_SUBSTITUTE)."\n");
        echo basename($file), ': ', count($data), " cases\n";
    }

    function tagged(mixed $v): mixed
    {
        if (is_array($v)) {
            $items = [];
            foreach ($v as $k => $x) {
                $items[] = [$k, tagged($x)];
            }

            return ['array' => $items];
        }

        return $v;
    }

    // --- comments ---
    $parser = (new ReflectionClass(RichParser::class))->newInstanceWithoutConstructor();
    (new ReflectionProperty(RichParser::class, 'ignoreLexer'))->setValue($parser, new IgnoreLexer());
    $linesToIgnore = new ReflectionMethod(RichParser::class, 'getLinesToIgnore');
    $lexer = new PhpParser\Lexer\Emulative();

    $codes = [];
    foreach (PHPStan\Parser\RichParserTest::dataLinesToIgnore() as [$code]) {
        $codes[] = $code;
    }
    foreach (PHPStan\Parser\RichParserTest::dataLinesToIgnoreParseErrors() as [$code]) {
        $codes[] = $code;
    }
    foreach (PHPStan\Analyser\Ignore\IgnoreLexerTest::dataTokenize() as [$input]) {
        $codes[] = "<?php\n/** @phpstan-ignore $input */\ntest();\n";
        $codes[] = "<?php\ntest(); // @phpstan-ignore $input\n";
    }
    array_push($codes,
        '',
        'just html',
        "<?php\n// @phpstan-ignore foo.bar\n",
        "<?php\n// @phpstan-ignore foo.bar",
        "<?php\ntest(); # @phpstan-ignore foo.bar, baz (because (nested) reasons), qux\n",
        "<?php\ntest(); /* @phpstan-ignore foo.bar () */\n",
        "<?php\ntest(); /* @phpstan-ignore foo.bar (unclosed */\n",
        "<?php\ntest(); /* @phpstan-ignore foo.bar) */\n",
        "<?php\ntest(); // @phpstan-ignore\n",
        "<?php\ntest(); // @phpstan-ignore foo-bar\n",
        "<?php\ntest(); // @phpstan-ignore foo.bar čičí\n",
        "<?php\ntest(); // @phpstan-ignore ,foo\n",
        "<?php\n/**\n * @phpstan-ignore foo.bar,\n *   baz.qux (comment\n *   continued)\n */\ntest();\n",
        "<?php\n/**\n * @phpstan-ignore foo.bar\n * @param int \$x\n */\nfunction f(\$x) {}\n",
        "<?php\n/**\n * @phpstan-ignore-next-line\n * @param int \$x\n */\nfunction f(\$x) {}\n",
        "<?php\n/**\n * @param int \$x\n * @phpstan-ignore-next-line\n */\nfunction f(\$x) {}\n",
        "<?php\n/** @phpstan-ignore-line */ test();\n/** @phpstan-ignore-line @phpstan-ignore-line */\ntest();\n",
        "<?php\n/**\n * @phpstan-ignore-next-line\n * @phpstan-ignore-next-line\n */\n\n\ntest();\n",
        "<?php\ntest(); // @phpstan-ignore-line\ntest(); // @phpstan-ignore foo\n// @phpstan-ignore-next-line\ntest(); // @phpstan-ignore bar\n",
        "<?php\ntest(); // @phpstan-ignore foo\n// @phpstan-ignore-next-line\ntest();\n",
        "<?php\n/* multi\n   @phpstan-ignore-next-line */\n\ntest();\n",
        "<?php\n// @phpstan-ignore foo.bar\n\n\ntest();\n",
        "<?php\n// @phpstan-ignore foo.bar\n// another comment\ntest();\n",
        "<?php\n// @phpstan-ignore foo.bar\n// @phpstan-ignore baz\ntest();\n",
        "<?php\n// @phpstan-ignore foo.bar,\ntest();\n",
        "<?php\n\$a = [\n    1, // @phpstan-ignore foo\n    2,\n];\n",
        "<?php\n\$x = <<<EOT\n// @phpstan-ignore not.a.comment\nEOT;\n// @phpstan-ignore real\nfoo();\n",
        "<?php\ntest();\r\n// @phpstan-ignore crlf (with reason)\r\ntest();\r\n",
        "<?php\n/**\r\n * @phpstan-ignore a.b,\r\n *   c.d\r\n */\r\ntest();\n",
        "<?php\n/** @phpstan-ignore a.b\t, c.d\t(x) */\ntest();\n",
        "<?php\n/** @phpstan-ignore a.b (x) c.d */\ntest();\n",
        "<?php\n/** @phpstan-ignore a.b (x)(y) */\ntest();\n",
        "<?php\n/** @phpstan-ignore a.b\r */\ntest();\n",
        "<?php\n/** @phpstan-ignored-thing foo */\ntest();\n",
        "<?php\ntest(); /** @Doctrine\\Tag @phpstan-ignore-next-line */\ntest();\n",
        "<?php\n?>html <?php // @phpstan-ignore x.y\n",
    );
    $comments = [];
    foreach ($codes as $code) {
        ['lines' => $lines, 'errors' => $errors] = $linesToIgnore->invoke($parser, $lexer->tokenize($code));
        $comments[] = ['code' => $code, 'lines' => tagged($lines), 'errors' => tagged($errors)];
    }
    write($fixtures.'/comments.json', $comments);

    // --- ignoreErrors ---
    $dir = $fixtures.'/ignore/files';
    $e = static fn (string $message, string $file = 'a.php', ?int $line = 1, ?string $identifier = null, bool $canBeIgnored = true, ?string $trait = null, ?string $description = null): Error => new Error(
        $message, $description ?? $dir.'/'.$file, $line, $canBeIgnored, $dir.'/'.$file, $trait === null ? null : $dir.'/'.$trait, identifier: $identifier,
    );
    $a = $dir.'/a.php';
    $b = $dir.'/b.php';
    $standardErrors = [$e('Foo'), $e('Foo', 'b.php', 2), $e('Bar baz', 'sub/c.php', 3, 'bar.baz'), $e('Qux', 'b.php', 4, 'qux.id')];
    $cases = [
        [['#^Foo$#'], true, $standardErrors],
        [['#^Foo$#', '#Nope#'], true, $standardErrors],
        [['#^Foo$#', '#Nope#'], false, $standardErrors],
        [['#^Foo$#', '#Nope#'], true, $standardErrors, true],
        [['#^Foo$#', '#Nope#'], true, $standardErrors, false, [], true],
        [[['message' => '#^Foo$#', 'path' => $a]], true, $standardErrors],
        [[['message' => '#^Foo$#', 'path' => $a], ['message' => '#^Nope$#', 'path' => $a]], true, $standardErrors, false, [$a]],
        [[['message' => '#^Nope$#', 'path' => $a]], true, $standardErrors, false, [$b]],
        [[['message' => '#^Nope$#', 'path' => $a]], true, $standardErrors, true, [$a]],
        [[['message' => '#^Foo$#', 'path' => $a, 'count' => 2]], true, [$e('Foo'), $e('Foo', 'a.php', 5)]],
        [[['message' => '#^Foo$#', 'path' => $a, 'count' => 2]], true, [$e('Foo', 'a.php', 7)]],
        [[['message' => '#^Foo$#', 'path' => $a, 'count' => 1]], true, [$e('Foo', 'a.php', 3), $e('Foo', 'a.php', 5)]],
        [[['message' => '#^Foo$#', 'path' => $a, 'count' => 2]], true, [$e('Foo', 'a.php', 3), $e('Foo', 'a.php', 5), $e('Foo', 'a.php', 6)]],
        [[['message' => '#^Foo$#', 'path' => $a, 'count' => 3]], true, [], false, [$a]],
        [[['message' => '#^Foo$#', 'path' => $a, 'count' => 3]], true, [$e('Foo')], true],
        [[['message' => '#^Foo$#', 'path' => $dir.'/nope.php', 'count' => 3]], true, [$e('Foo')], true],
        [[['message' => '#^Foo$#', 'path' => $dir.'/nope.php', 'count' => 3]], true, [$e('Foo')]],
        [[['message' => '#^Foo$#', 'path' => $dir.'/nope.php', 'count' => 3]], true, []],
        [[['message' => '#^Foo$#', 'path' => $dir.'/nope.php']], true, [], true],
        [[['message' => '#^Foo$#', 'path' => $a, 'count' => 1], ['message' => '#^Foo$#', 'path' => $a, 'count' => 2]], true, [$e('Foo'), $e('Foo'), $e('Foo'), $e('Foo')]],
        [[['message' => '#^Foo$#', 'path' => $a], ['message' => '#^Foo$#', 'path' => $a, 'reportUnmatched' => false]], true, []],
        [[['message' => '#^Foo$#', 'path' => $a, 'reportUnmatched' => false], ['message' => '#^Foo$#', 'path' => $a]], true, [], false, [$a]],
        [[['message' => '#^Foo$#', 'path' => $a, 'reportUnmatched' => false], ['message' => '#^Foo$#', 'path' => $a, 'reportUnmatched' => false]], true, [], false, [$a]],
        [[['rawMessage' => 'Foo', 'path' => $a, 'identifier' => 'x.y'], ['rawMessage' => 'Foo', 'path' => $a, 'identifier' => 'x.y']], true, [], false, [$a]],
        [[['rawMessage' => 'Foo', 'path' => $a], ['identifier' => 'qux.id', 'path' => $b]], true, $standardErrors, false, [$a, $b]],
        [[['rawMessage' => 'Foo'], ['rawMessage' => 'Nope']], true, $standardErrors],
        [[['identifier' => 'bar.baz'], ['identifier' => 'nope.id']], true, $standardErrors],
        [[['identifiers' => ['bar.baz', 'qux.id', 'nope.id']]], true, $standardErrors],
        [[['identifiers' => ['bar.baz', 'nope.id'], 'path' => $b]], true, $standardErrors, false, [$b]],
        [[['messages' => ['#^Foo$#', '#^Nope$#'], 'path' => $a]], true, $standardErrors, false, [$a]],
        [[['rawMessages' => ['Qux', 'Nope'], 'paths' => [$b, $dir.'/sub/*']]], true, $standardErrors],
        [[['rawMessages' => ['Bar baz', 'Nope'], 'identifier' => 'bar.baz', 'paths' => [$dir.'/sub/*']]], true, $standardErrors],
        [[['message' => '#^Foo$#', 'paths' => [$a, $b, $dir.'/sub']]], true, $standardErrors],
        [[['message' => '#.#', 'paths' => [$dir.'/*.php']]], true, $standardErrors],
        [[['message' => '#^Foo$#', 'paths' => []]], true, $standardErrors],
        [[['message' => '#^Bar#', 'path' => $dir.'/sub']], true, $standardErrors],
        [[['message' => '#^Bar#', 'path' => $dir.'/sub/']], true, $standardErrors],
        [[['message' => '#^Bar#', 'path' => $dir.'/s?b/*.php']], true, $standardErrors],
        [[['message' => '#^Bar#', 'path' => $dir.'/./sub/../sub/c.php']], true, $standardErrors],
        [[['message' => '#^Nope#', 'path' => $dir.'/./sub/../sub/c.php']], true, $standardErrors, false, [$dir.'/sub/c.php']],
        [[['path' => "q\\\"/\x08\x0c\x01\u{2028}\u{2029}é\n\r\t", 'reportUnmatched' => false, 'count' => 3, 'x' => null]], true, []],
        [[['path' => $a], ['paths' => [$a]], ['messages' => [], 'path' => $a], ['message' => '#^Foo$#', 'count' => 1, 'path' => $a, 'reportUnmatched' => true, 'x' => [1, 'a' => null, true, false, 'é/"\\']]], true, []],
        [['#^Foo$#'], true, [$e('Foo', canBeIgnored: false)]],
        [[['message' => '#^Foo$#', 'path' => $a]], true, [$e('Foo', canBeIgnored: false)]],
        [[['message' => '#^Foo$#', 'path' => $dir.'/trait.php']], true, [$e('Foo', trait: 'trait.php'), $e('Foo', 'b.php', trait: 'sub/c.php')]],
        [[['message' => '#^Foo$#', 'path' => $dir.'/t*.php']], true, [$e('Foo', trait: 'trait.php')]],
        [[['message' => '#^Foo$#', 'path' => $dir.'/t*.php', 'count' => 1]], true, [$e('Foo', trait: 'trait.php', description: 'a.php (in context of class X)')]],
        [['#^Line\\\\nbreak$#', '#^Other\\\\r\\\\nbreak$#', '#^Third\\\\rx$#'], true, [$e('Line\\r\\nbreak'), $e('Other\\nbreak'), $e('Third\\nx'), $e("Real\r\nnewline")]],
        [['#^Real\\r\\nnewline$#', '#^Real\\nnewline$#'], true, [$e("Real\r\nnewline")]],
        [['~^foo$~i', '#unicode é#u'], true, [$e('FOO'), $e('unicode é')]],
        [[['message' => '#^Foo$#', 'identifier' => 'bar.baz']], true, [$e('Foo', identifier: 'bar.baz'), $e('Foo')]],
        [[['message' => '#^Foo$#', 'rawMessage' => 'Foo', 'path' => $a]], true, [$e('Foo')]],
        [[['rawMessage' => 'Foo', 'path' => $a, 'count' => 2], ['rawMessage' => 'Foo', 'path' => $b]], true, [$e('Foo'), $e('Foo'), $e('Foo')], false, [$a, $b]],
        [[['message' => '#^Foo$#', 'identifier' => 'x.y', 'path' => $a, 'count' => 1]], true, [$e('Foo', identifier: 'x.y'), $e('Foo', 'a.php', 9, 'x.y')]],
        [[['identifier' => 'x.y', 'path' => $a, 'count' => 1], ['message' => '#^Bar baz$#', 'rawMessage' => 'Bar baz', 'paths' => [$dir.'/sub/c.php']]], true, [$e('Foo', identifier: 'x.y'), $e('Foo', 'a.php', 9, 'x.y'), $e('Foo', 'a.php', null, 'x.y')], false, [$a]],
        [[['rawMessage' => 'Nope', 'paths' => [$a]], ['rawMessage' => 'Nope', 'paths' => [$a, $b]], ['rawMessage' => 'Nope', 'identifier' => 'a.b']], true, []],
        [[['message' => '#^Foo$#', 'path' => $a, 'count' => 2]], true, [$e('Foo')], false, [], true],
    ];
    $out = [];
    $dump = static fn (Error $x): array => ['message' => $x->getMessage(), 'file' => $x->getFile(), 'line' => $x->getLine(), 'canBeIgnored' => $x->canBeIgnored(), 'filePath' => $x->getFilePath(), 'traitFilePath' => $x->getTraitFilePath(), 'tip' => $x->getTip(), 'identifier' => $x->getIdentifier()];
    $placeholder = static fn (mixed $v) => json_decode(str_replace(addcslashes($dir, '/'), '{dir}', json_encode($v)), true);
    foreach ($cases as $case) {
        [$ignoreErrors, $reportUnmatched, $errors, $onlyFiles, $analysedFiles, $internal] = $case + [3 => false, 4 => [], 5 => false];
        $helper = (new IgnoredErrorHelper(new FileHelper($root), $ignoreErrors, $reportUnmatched))->initialize();
        $result = $helper->process($errors, $onlyFiles ?? false, $analysedFiles ?? [], $internal ?? false);
        $out[] = $placeholder([
            'ignoreErrors' => tagged($ignoreErrors),
            'reportUnmatched' => $reportUnmatched,
            'errors' => array_map($dump, $errors),
            'onlyFiles' => $onlyFiles ?? false,
            'analysedFiles' => $analysedFiles ?? [],
            'hasInternalErrors' => $internal ?? false,
            'expected' => [
                'configErrors' => $helper->getErrors(),
                'errors' => array_map($dump, $result->getNotIgnoredErrors()),
                'ignored' => array_map(static fn (array $p) => [$dump($p[0]), tagged($p[1])], $result->getIgnoredErrors()),
                'stringErrors' => $result->getOtherIgnoreMessages(),
            ],
        ]);
    }
    write($fixtures.'/ignore.json', $out);

    // --- LocalIgnoresProcessor ---
    $processor = new PHPStan\Analyser\LocalIgnoresProcessor();
    $local = [];
    $localCases = [
        ["<?php\na(); // @phpstan-ignore x.a, x.b\nb(); // @phpstan-ignore-line\nc(); // @phpstan-ignore x.c (why)\n", [[2, 'x.a'], [2, null], [2, 'x.b'], [2, 'x.b'], [3, null], [3, 'q.q'], [4, 'x.c', false], [4, 'x.c', true, 'other.php'], [4, 'x.c'], [5, 'x.c'], [null, 'x.c']]],
        ["<?php\na(); // @phpstan-ignore x.a, x.a, x.b\n", [[2, 'x.a'], [2, 'x.a'], [2, 'x.a']]],
        ["<?php\na(); // @phpstan-ignore x.a\n", [[2, 'x.a']], true], // unmatchedLineIgnores already without the line
    ];
    foreach ($localCases as $localCase) {
        [$code, $errs, $noUnmatched] = $localCase + [2 => false];
        ['lines' => $lines] = $linesToIgnore->invoke($parser, $lexer->tokenize($code));
        $errors = array_map(fn (array $x) => new Error('E', $x[3] ?? 'f.php', $x[0], $x[2] ?? true, identifier: $x[1]), $errs);
        $r = $processor->process($errors, ['f.php' => $lines], ['f.php' => $noUnmatched ? [] : $lines]);
        $local[] = [
            'code' => $code,
            'noUnmatched' => $noUnmatched,
            'errors' => array_map($dump, $errors),
            'kept' => array_map($dump, $r->getFileErrors()),
            'ignored' => array_map($dump, $r->getLocallyIgnoredErrors()),
            'lines' => tagged($r->getLinesToIgnore()['f.php'] ?? []),
            'unmatched' => tagged($r->getUnmatchedLineIgnores()['f.php'] ?? []),
        ];
    }
    write($fixtures.'/local-processor.json', $local);
}
