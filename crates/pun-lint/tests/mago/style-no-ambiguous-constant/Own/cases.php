<?php
namespace App;
use const Foo\BAR;
use const Foo\{BAZ as Q};
echo BAR, bar, Q, BAZ, FOO, \FOO, Sub\FOO, namespace\FOO, TRUE, Null, STDIN, stderr, PHP_EOL;
function f($x = PHP_INT_MAX) { return M_PI; }
