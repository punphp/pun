<?php
// first
$a = $a;
$b = &$b;
echo $b;
-----
<?php
$b = &$b;
echo $b;
