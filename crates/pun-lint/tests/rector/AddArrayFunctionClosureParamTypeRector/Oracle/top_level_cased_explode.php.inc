<?php

$s = strtolower($argv[1]);
array_map(fn ($p) => $p, explode('-', $s));
array_map(fn ($p) => $p, explode('-', strtolower($argv[1])));
$parts = explode('-', $s);
array_map(fn ($p) => $p, $parts);
-----
<?php

$s = strtolower($argv[1]);
array_map(fn (string $p) => $p, explode('-', $s));
array_map(fn (string $p) => $p, explode('-', strtolower($argv[1])));
$parts = explode('-', $s);
array_map(fn (string $p) => $p, $parts);
