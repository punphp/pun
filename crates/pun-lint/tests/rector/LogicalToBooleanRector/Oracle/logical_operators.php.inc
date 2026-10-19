<?php

$r = $a instanceof B and $c;
$r = ++$a and $b;
$r = $a and ($b ? 1 : 2);
$r = $a and !($b ?: 2);
$r = $a and $b ? 1 : 2;
$r = $a and -($b ? 1 : 2);
$r = $a and ($b ? 1 : 2) . 'x';
$r = $a and ($b ?: 2) * 3;
-----
<?php

($r = $a instanceof B) && $c;
$r = ++$a and $b;
$r = $a and ($b ? 1 : 2);
$r = $a and !($b ?: 2);
$r = $a and $b ? 1 : 2;
$r = $a and -($b ? 1 : 2);
$r = $a and ($b ? 1 : 2) . 'x';
$r = $a and ($b ?: 2) * 3;
