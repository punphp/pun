<?php

$a or foo(1,
   2);
$a or ($b ||
    $c);
$a or ($b   ||  $c);
$a or $b ||
    $c;
$a or ($b
   && $c);
$a   or   $b;
$a or /* x */ $b;
$a or [1,
  2];
$a or $b and
  $c;
foo(1,
  2) or $a;
$a or ($b === 1
  ? 2 : 3);
$a or (int)   $b;
$q = $a and $b and $c;
$ok = true or die();
$r = isset($x) and $y;
-----
<?php

$a || foo(1,
   2);
$a || ($b || $c);
$a || ($b || $c);
$a || ($b || $c);
$a || $b && $c;
$a || $b;
$a || $b;
$a || [1,
  2];
$a || $b && $c;
foo(1,
  2) || $a;
$a || ($b === 1 ? 2 : 3);
$a || (int)   $b;
($q = $a) && $b && $c;
($ok = true) || die();
($r = isset($x)) && $y;
