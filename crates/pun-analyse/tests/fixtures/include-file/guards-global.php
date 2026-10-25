<?php
if (file_exists('guarded.php')) { require 'guarded.php'; }
if (is_file('guarded.php')) { require 'guarded.php'; }
if (is_readable('guarded.php')) { require 'guarded.php'; }
if (is_writable('guarded.php')) { require 'guarded.php'; }
if (is_writeable('guarded.php')) { require 'guarded.php'; }
if (is_executable('guarded.php')) { require 'guarded.php'; }
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
function includeVariables($path): void {
    if (file_exists($path)) { require $path; }
    if (!file_exists($path)) { require $path; }
    if (file_exists($path)) {
        $path = 'changed.php';
        require $path;
    }
}

function includeEarly(): void {
    if (!is_readable('early.php')) { return; }
    include 'early.php';
}

$ok = file_exists('aliased.php');
if ($ok) { include 'aliased.php'; }
if (file_exists(__DIR__ . '/guarded.php')) { require __DIR__.'/guarded.php'; }
if (file_exists('first.php')) {
    include 'first.php';
} elseif (is_file('second.php')) {
    include 'second.php';
} else {
    include 'third.php';
}

if (file_exists('same.php') || is_file('same.php')) { include 'same.php'; }
if (file_exists('same.php') xor is_file('different.php')) { include 'same.php'; }
if (file_exists('compare.php') === random_int(0, 1)) { include 'compare.php'; }
if (file_exists('double.php')) { include "double.php"; }
if (file_exists('brace.php') == true) { include 'brace.php'; }
if (true == is_file('brace.php')) { include 'brace.php'; }
if (false == is_file('brace.php')) {} else { include 'brace.php'; }
if (is_file('brace.php') != false) { include 'brace.php'; }
if (is_file('brace.php') !== true) {} else { include 'brace.php'; }
if (file_exists('identical.php') || file_exists('identical.php')) { include 'identical.php'; }

class IncludeConstructor {
    public function __construct() {
        if (!file_exists('constructor.php')) { throw new RuntimeException(); }
        include 'constructor.php';
    }
    public function later(): void { include 'constructor.php'; }
}
