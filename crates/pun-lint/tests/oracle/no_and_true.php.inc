<?php
$a = true && ($b || $c);
$a = ($b ?? $c) && TRUE;
$a = \true && $b;
$a = $b && (true && true);
$a = true and $b;
$a = foo(true && ($b or $c));
$a = (true && $b) . 'x';
$a = $b && $c && true;
$a = true && $b = 5;
-----
<?php
$a = $b || $c;
$a = $b ?? $c;
$a = $b;
$a = $b;
$a = true and $b;
$a = foo($b or $c);
$a = ($b) . 'x';
$a = $b && $c;
$a = $b = 5;
