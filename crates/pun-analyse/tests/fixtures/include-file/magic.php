<?php
namespace IncludeFileMagic;

include __DIR__ . '/missing.php';
include (__DIR__) . '/missing-parenthesized.php';
include __DIR__ . '/script.txt';
include __FILE__;
include __FILE__ . '.missing';
$path = '/missing-variable.php';
include __DIR__ . $path;
$part = 'missing';
$file = 'variable.php';
include __DIR__ . '/' . $part . '/' . $file;
include __DIR__ . "/$part/$file";
include __DIR__ . DIRECTORY_SEPARATOR . $part . '/' . $file;
include ('a' . DIRECTORY_SEPARATOR . 'b');
include 'missing' . '\\' . 'separator.php';
include 'missing//' . 'separator.php';
include '/prefix' . (__DIR__ . '/suffix.php');
include __DIR__ . '/a.php' . (__DIR__ . '/b.php');

/** @param '/one.php'|'/two.php' $path */
function alternatives($path): void { include __DIR__ . $path; }
function unknown(string $path): void { include __DIR__ . $path; }

$absolute = __DIR__ . '/assigned.php';
include $absolute;
include dirname(__FILE__) . '/dirname.php';

include __DIR__ . '/' . ('missing' . '.php');
include __DIR__ . '/quoted"file.php';
include __DIR__ . "/{$part}/{$file}";
$directory = __DIR__;
include $directory . '/copied.php';
$copy = $directory;
include $copy . '/copied-twice.php';
include __DIR__ . (null . '/null.php');

$joined = __DIR__ . '/script.txt';
if (random_int(0, 1)) { $joined = 'missing-joined.php'; }
include $joined;
$joined = 'missing-reset.php';
include $joined;

$paths = [__DIR__ . '/missing-array.php', 'missing-literal-array.php'];
include $paths[0];
include $paths[1];
