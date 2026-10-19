<?php

$a = $$name;
$b = $$$name;
$c = ${'x'};
$d = ${$x . 'y'};
$e = $plain;
-----
3:6: Usage of variable variables (e.g., `$$var`) is discouraged.
4:6: Usage of variable variables (e.g., `$$var`) is discouraged.
4:7: Usage of variable variables (e.g., `$$var`) is discouraged.
