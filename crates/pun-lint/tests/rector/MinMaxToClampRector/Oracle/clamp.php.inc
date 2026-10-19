<?php

max(0, min(100, $v, 3));
max(...$a);
min(100, max(-5, $v));
max(0.5, min($v, 'z'));
max(null, min(true, $v));
max(0, max(100, $v));
max(x: 0, y: min(100, $v));
max(0, min(...$v));
max(0, min(100, $w));
max($lo, min($v, $hi));
max(PHP_FLOAT_EPSILON, min(PHP_INT_SIZE, $v));
\max(0, \min(100, $v));
max(0, (min(100, $v)));
-----
<?php

max(0, min(100, $v, 3));
max(...$a);
clamp($v, -5, 100);
clamp($v, 0.5, 'z');
clamp($v, null, true);
max(0, max(100, $v));
max(x: 0, y: min(100, $v));
max(0, min(...$v));
clamp($w, 0, 100);
max($lo, min($v, $hi));
clamp($v, PHP_FLOAT_EPSILON, PHP_INT_SIZE);
clamp($v, 0, 100);
clamp($v, 0, 100);
