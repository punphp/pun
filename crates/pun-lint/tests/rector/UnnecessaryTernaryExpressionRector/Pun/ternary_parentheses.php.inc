<?php

$a = !($b = foo()) ? false : true;
$a = ($b = foo()) ? false : true;
$a = (fn () => 1) ? false : true;
$a = (print 1) ? false : true;
$a = !(include 'a.php') ? false : true;
-----
<?php

$a = (bool) ($b = foo());
$a = !(bool) ($b = foo());
$a = !(bool) (fn () => 1);
$a = !(bool) (print 1);
$a = (bool) (include 'a.php');
