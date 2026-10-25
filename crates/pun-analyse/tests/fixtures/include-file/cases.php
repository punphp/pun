<?php
namespace IncludeFile;

include 'missing.php';
include_once 'missing-once.php';
require 'missing-required.php';
require_once 'missing-required-once.php';
include 'script.txt';
include '.';
include '';
include 'vfs://unavailable/file.php';
include 'NOT-A-WRAPPER://missing.php';
include 'file://' . __DIR__ . '/script.txt';
include 'file://' . __DIR__ . '/missing.php';

/** @param 'missing-a.php'|'script.txt'|'missing-b.php' $path */
function alternatives($path): void {
    require $path;
}

/** @param 'missing-null.php'|null $path */
function nullable($path): void {
    include $path;
}

function unknown(string $path, bool $flag): void {
    include $path;
    include ($flag ? 'script.txt' : 'missing-conditional.php');
}

$directory = 'folder';
$file = 'missing.php';
include $directory . '/' . $file;
include 'folder/../script.txt';

/** @return 'missing-return.php' */
function path(): string { return 'missing-return.php'; }
include path();

// @phpstan-ignore include.fileNotFound
include 'ignored.php';

include 123;
include null;
include false;

print 1;
include (1 + 2);
include 'a' . PATH_SEPARATOR . 'b';
include '\script.txt';
include 'file://script.txt';

/**
 * @template T of 'template-one.php'|'template-two.php'
 * @param T $path
 */
function templatePath($path): void { include $path; }

/** @param ('intersection.php'&non-empty-string) $path */
function intersectionPath($path): void { include $path; }

namespace IncludeFileSeparatorShadow;
const DIRECTORY_SEPARATOR = '-';
const PATH_SEPARATOR = '#';
include 'a' . DIRECTORY_SEPARATOR . 'b';
include 'a' . PATH_SEPARATOR . 'b';
