<?php

$x = ['a', 1];
array_filter($x, fn ($v) => $v !== 1);

$s = strtolower($argv[1]);
array_map(fn ($p) => $p, explode('-', $s));
-----
<?php

$x = ['a', 1];
array_filter($x, fn (string|int $v) => $v !== 1);

$s = strtolower($argv[1]);
array_map(fn (string $p) => $p, explode('-', $s));
