<?php

$a = $b === 1 ? false : true;
$a = $b !== 1 ? false : true;
$a = $b == 1 ? false : true;
$a = $b != 1 ? false : true;
$a = $b <> 1 ? false : true;
$a = $b > 1 ? false : true;
$a = $b < 1 ? false : true;
$a = $b >= 1 ? false : true;
$a = $b <= 1 ? false : true;
$a = $b && $c ? false : true;
$a = $b ? true : true;
$a = $b == 1 ? true : true;
$a = $b ? false : false;
$a = ($b |> foo(...)) ? true : false;
$a = ($b |> foo(...)) ? false : true;
-----
<?php

$a = $b !== 1;
$a = $b === 1;
$a = $b != 1;
$a = $b == 1;
$a = $b == 1;
$a = $b <= 1;
$a = $b >= 1;
$a = $b < 1;
$a = $b > 1;
$a = $b && $c ? false : true;
$a = $b ? true : true;
$a = $b != 1;
$a = $b ? false : false;
$a = $b |> foo(...);
$a = ($b |> foo(...)) ? false : true;
