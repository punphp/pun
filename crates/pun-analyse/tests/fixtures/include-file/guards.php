<?php
namespace IncludeFileGuards;

use function is_file as present;

if (file_exists('guarded.php')) { require 'guarded.php'; }
if (is_file('guarded.php')) { require 'guarded.php'; }
if (is_readable('guarded.php')) { require 'guarded.php'; }
if (is_writable('guarded.php')) { require 'guarded.php'; }
if (is_writeable('guarded.php')) { require 'guarded.php'; }
if (is_executable('guarded.php')) { require 'guarded.php'; }
if (present('guarded.php')) { require 'guarded.php'; }
if (\FILE_EXISTS('guarded.php')) { require 'guarded.php'; }
if (file_exists('guarded.php') === true) { require 'guarded.php'; }
if (false !== file_exists('guarded.php')) { require 'guarded.php'; }
if (!file_exists('guarded.php')) { require 'guarded.php'; }
if (file_exists('other.php')) { require 'guarded.php'; }
if (is_dir('guarded.php')) { require 'guarded.php'; }
if (file_exists('guarded.php') || random_int(0, 1)) { require 'guarded.php'; }
if (file_exists('guarded.php') && random_int(0, 1)) { require 'guarded.php'; }
file_exists('guarded.php') && include 'guarded.php';
file_exists('guarded.php') || include 'guarded.php';
!file_exists('guarded.php') || include 'guarded.php';
file_exists('guarded.php') ? include 'guarded.php' : include 'guarded.php';

/** @param 'a.php'|'b.php' $path */
function variables($path): void {
    if (file_exists($path)) { require $path; }
    if (!file_exists($path)) { require $path; }
    if (file_exists($path)) {
        $path = 'changed.php';
        require $path;
    }
}

function early(): void {
    if (!is_readable('early.php')) { return; }
    include 'early.php';
}

$ok = file_exists('aliased.php');
if ($ok) { include 'aliased.php'; }

if (file_exists(__DIR__ . '/guarded.php')) {
    require __DIR__.'/guarded.php';
} elseif (file_exists(__DIR__ . '/another.php')) {
    require __DIR__ . '/another.php';
}

namespace IncludeFileShadow;
function file_exists(string $path): bool { return true; }
if (file_exists('shadow.php')) { require 'shadow.php'; }
if (\file_exists('shadow.php')) { require 'shadow.php'; }
