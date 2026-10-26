<?php

$a = fn (array $items) => ! empty($items);
$b = fn (array $items) => [! empty($items)];
$c = fn (array $items) => empty($items);
$d = fn (array $items) => (! empty($items)) && true;
$e = function (array $items) { return ! empty($items); };
-----
<?php

$a = fn (array $items) => $items !== [];
$b = fn (array $items) => [$items !== []];
$c = fn (array $items) => $items === [];
$d = fn (array $items) => ($items !== []) && true;
$e = function (array $items) { return $items !== []; };
