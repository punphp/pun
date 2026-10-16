<?php

declare(strict_types=1);

/*
 * The PHP and PHPStan helpers crates/pun-phpstan/src/php.rs reimplements, run for real (PHP + the PHPStan 2.2.7
 * phar's classes), written to crates/pun-phpstan/tests/fixtures/php.json:
 *
 *   regex           Nette Strings::match($subject, $pattern): matched, or the RegexpException message
 *   preg_quote      preg_quote($s, $delimiter)
 *   normalize_path  FileHelper::normalizePath()
 *   absolutize_path FileHelper(cwd)->absolutizePath()
 *   relative_path   ParentDirectoryRelativePathHelper($parent)->getRelativePath($file)
 *   fnmatch         fnmatch($pattern, $string)
 *
 *   php scripts/export-php-helpers-fixtures.php
 */

$root = dirname(__DIR__);
Phar::loadPhar($root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar', 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use _PHPStan_88a9c7bcc\Nette\Utils\Strings;
use PHPStan\File\FileHelper;
use PHPStan\File\ParentDirectoryRelativePathHelper;

$regex = [];
$patterns = [
    ['#^Foo$#', 'Foo'], ['#^Foo$#', 'foo'], ['#^foo$#i', 'FOO'], ['/a.c/s', "a\nc"], ['/a.c/', "a\nc"], ['/^b$/m', "a\nb"],
    ['/a b/x', 'ab'], ['/é/u', 'é'], ['/^.$/u', 'é'], ['/^.$/', 'é'], ['/\w/u', 'é'], ['/b/A', 'ab'], ['/a+/U', 'aaa'],
    ['/(?<n>a)|(?<n>b)/J', 'b'], ['/(a)/n', 'a'], ['/a$/D', "a\n"], ['/a/S', 'a'], ['/a/X', 'a'],
    ["  \n/a/", 'a'], ['(a)', 'a'], ['[a]i', 'A'], ['{a{1}}', 'a'], ['<a>', 'a'], ['(a(b)c)', 'abc'], ['#a\#b#', 'a#b'],
    ['', 'a'], ['   ', 'a'], ['abc', 'a'], ['\\a\\', 'a'], ['/abc', 'a'], ['(abc', 'a'], ['{a', 'a'], ['/a/Z', 'a'], ['/a/ i', 'a'],
    ["/a/\0", 'a'], ['/a(/', 'a'], ['/a)/', 'a'], ['/[a/', 'a'], ['/a{2,1}/', 'a'], ['/(?<1a>x)/', 'x'], ['/a/A', 'ba'],
    ['/a(/A', 'a'], ['/x(/i', 'a'], ['/\p{Zz}/u', 'a'], ["/\xff/u", 'a'], ['/a/u', "\xff"], ['~^\d+~', '123'], ['/a #c/xA', 'a'], ['/a #c/xA', 'ba'],
];
foreach ($patterns as [$pattern, $subject]) {
    try {
        $r = ['pattern' => $pattern, 'subject' => $subject, 'matched' => Strings::match($subject, $pattern) !== null];
    } catch (Throwable $e) {
        $r = ['pattern' => $pattern, 'subject' => $subject, 'error' => $e->getMessage()];
    }
    if (!mb_check_encoding($pattern.$subject, 'UTF-8')) {
        continue; // Rust strings are UTF-8
    }
    $regex[] = $r;
}

$quote = [];
foreach ([['a.b*c?d+e(f)g[h]i{j}k=l!m<n>o|p:q-r#s/t\\u^v$w', null], ['a/b#c', '/'], ['a/b#c', '#'], ["nul\0byte", null], ['~x~', '~'], ['é', null]] as [$s, $d]) {
    $quote[] = ['s' => $s, 'delimiter' => $d, 'quoted' => preg_quote($s, $d)];
}

$helper = new FileHelper('/cwd/project');
$normalize = [];
foreach (['/a/b/../c', '/a/./b//c/', 'a\\b\\c', '../x', '/../x', 'phar:///x/y.phar/../z', 'phar:///x/y.phar/a/../b', 'PHAR:///a/./b', 'C:\\a\\..\\b', 'vfs://root/a/../b', '1x://a', 'a://', '://a', '', '/', 'a/b/../../..'] as $p) {
    $normalize[] = ['path' => $p, 'normalized' => $helper->normalizePath($p)];
}
$absolutize = [];
foreach (['/abs', 'rel/x', './x', '../x', 'phar://x/y', 'a+b.c-d://x', 'bad scheme://x', '\\x'] as $p) {
    $absolutize[] = ['path' => $p, 'absolute' => $helper->absolutizePath($p)];
}
$relative = [];
foreach ([['/a/b', '/a/b/c.php'], ['/a/b', '/a/c/d.php'], ['/a/b', '/x/y.php'], ['/a/b/', '/a/b/c.php'], ['/a', 'rel.php'], ['/a/b', 'phar:///a/b/c.php'], ['/a/b/c', '/a/b'], ['\\a\\b', '/a/b/c.php']] as [$parent, $file]) {
    $relative[] = ['parent' => $parent, 'file' => $file, 'relative' => (new ParentDirectoryRelativePathHelper($parent))->getRelativePath($file)];
}
$fnmatch = [];
foreach ([['*.php', 'a.php'], ['/a/*/c', '/a/b/c'], ['/a/*', '/a/b/c'], ['[ab]?', 'bc'], ['a\\*', 'a*']] as [$p, $s]) {
    $fnmatch[] = ['pattern' => $p, 'string' => $s, 'matched' => @fnmatch($p, $s)];
}

file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/php.json', json_encode(compact('regex', 'quote', 'normalize', 'absolutize', 'relative', 'fnmatch'), JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
echo count($regex), " regexes\n";
