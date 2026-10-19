<?php

namespace App;

use function Symfony\Component\VarDumper\dump as sdump;

var_dump($a);
\var_dump($a);
print_r($a);
print_r($a, true);
var_export($a, true);
var_export($a);
dd($a);
DD($a);
dump($a, $b);
sdump($a);
\Mago\inspect($a);
phpstan\dumpType($a);
\PHPStan\dumpType($a);
xdebug_break();
ray($a);
debug_zval_dump($a);
$x = phpinfo();
$obj->dump($a);
Foo::dd();
expect($a)->dump()->toBe(1);
-----
7:1: Do not commit debug functions.
8:1: Do not commit debug functions.
9:1: Do not commit debug functions.
12:1: Do not commit debug functions.
13:1: Do not commit debug functions.
14:1: Do not commit debug functions.
15:1: Do not commit debug functions.
17:1: Do not commit debug functions.
18:1: Do not commit debug functions.
19:1: Do not commit debug functions.
20:1: Do not commit debug functions.
22:1: Do not commit debug functions.
23:6: Do not commit debug functions.
