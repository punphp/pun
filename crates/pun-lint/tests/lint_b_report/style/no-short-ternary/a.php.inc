<?php

$a = $b ?: $c;
$d = $e ? $f : $g;
$h = $i
    ?: $j ?: $k;
$l = $m ?? $n;
-----
3: Short ternary operator is not allowed. Use null coalesce operator if applicable or consider using long ternary.
5: Short ternary operator is not allowed. Use null coalesce operator if applicable or consider using long ternary.
5: Short ternary operator is not allowed. Use null coalesce operator if applicable or consider using long ternary.
