<?php

namespace App;

use Foo;
use const Foo\BAR;
use function explode as split;
use function Foo\{bar, explode as boom};
use Baz\{Qux, function explode as blow, const QUUX};

$a = split($s, '-');
$b = explode($s, '-');
$c = boom($s, '-');
$d = Foo\explode($s, '-');
$e = blow($s, '-');
$f = $fn($s, '-');
$g = late($s, '-');

use function explode as late;
-----
11:16: The arguments to `explode` may be swapped.
12:18: The arguments to `explode` may be swapped.
