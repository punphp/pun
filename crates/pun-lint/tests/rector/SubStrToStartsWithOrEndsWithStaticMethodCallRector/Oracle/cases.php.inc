<?php

$a = substr($v, -2) == $x;
$b = substr($v, +0) === 'a';
$c = substr($v, '0') === 'a';
$d = substr($v, 0) !== 'a';
$e = SUBSTR($v, - 3) === $f . $g;
$h = substr($v) === 'a';
$i = (substr($v, 0)) === ($w);
-----
<?php

$a = \Illuminate\Support\Str::endsWith($v, $x);
$b = \Illuminate\Support\Str::startsWith($v, 'a');
$c = substr($v, '0') === 'a';
$d = substr($v, 0) !== 'a';
$e = \Illuminate\Support\Str::endsWith($v, $f . $g);
$h = substr($v) === 'a';
$i = \Illuminate\Support\Str::startsWith($v, $w);
