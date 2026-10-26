<?php
namespace App;
use function Foo\check as assert;
assert($a);
\assert($a);
ASSERT($a, 'x');
App\assert($a);
function f() { assert(true); }
