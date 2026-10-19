<?php

if (empty($a)) {
}
$b = !empty($c['x']) && empty(
    $d
);
$e = isset($f);
-----
3: Construct empty() is not allowed. Use more strict comparison.
5: Construct empty() is not allowed. Use more strict comparison.
5: Construct empty() is not allowed. Use more strict comparison.
