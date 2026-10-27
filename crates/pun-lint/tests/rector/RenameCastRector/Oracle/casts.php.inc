<?php

$a = (integer)$b;
$a = (integer) (($b));
$a = (integer)-$b;
$a = (integer) $b + 1;
$a = ( INTEGER ) $b;
$a = (int) $b;
$a = (double) /* c */ $b;
$a = (DOUBLE)(integer)$b;
$a = (integer) $b ** 2;
$a = (integer) $b instanceof C;
$a = (boolean) $b;
$a = (float) $b;
-----
<?php

$a = (int) $b;
$a = (int) $b;
$a = (int) -$b;
$a = (int) $b + 1;
$a = (int) $b;
$a = (int) $b;
$a = (float) $b;
$a = (float) (int) $b;
$a = (int) $b ** 2;
$a = (int) $b instanceof C;
$a = (boolean) $b;
$a = (float) $b;
