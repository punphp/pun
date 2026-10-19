<?php

namespace App;

use function Foo\explode;

$a = explode($s, '-');
$b = \explode($s, '-');
-----
8:19: The arguments to `explode` may be swapped.
