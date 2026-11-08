<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function f($k, $keys, $m, mixed $x, ?string $s, int|null $i, $arr)
{
    array_key_exists($k, $arr);
    array_key_exists($keys, $arr);
    array_key_exists($m, $arr);
    array_key_exists($x, $arr);
    array_key_exists($s, $arr);
    array_key_exists($i, $arr);
    array_key_exists($undefined, $arr);
    array_key_exists(NULL, $arr);
    array_key_exists(\null, $arr);
    array_key_exists($k ? $x : null, $arr);
    array_key_exists($k ? $x : $m, $arr);
    array_key_exists($k ? null : $m, $arr);
    array_key_exists((string) $k ? $x : $m, $arr);
    array_key_exists((string) $k ? 'a' : 'b', $arr);
    array_key_exists($s ?: 'x', $arr);
    array_key_exists($arr[0], $arr);
    array_key_exists("a$k", $arr);
    array_key_exists($k ?? null, $arr);
    array_key_exists(($k), $arr);
    array_key_exists(...$arr);
    array_key_exists($k, $arr, $m);
    \array_key_exists($m, $arr);
    array_key_exists(array: $arr, key: $m);
}

array_key_exists($top, $arr);
-----
<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function f($k, $keys, $m, mixed $x, ?string $s, int|null $i, $arr)
{
    array_key_exists($k, $arr);
    array_key_exists($keys, $arr);
    array_key_exists($m, $arr);
    array_key_exists($x, $arr);
    array_key_exists($s, $arr);
    array_key_exists($i, $arr);
    array_key_exists($undefined, $arr);
    array_key_exists('', $arr);
    array_key_exists('', $arr);
    array_key_exists($k ? $x : null, $arr);
    array_key_exists($k ? $x : $m, $arr);
    array_key_exists($k ? null : $m, $arr);
    array_key_exists((string) $k ? $x : $m, $arr);
    array_key_exists((string) $k ? 'a' : 'b', $arr);
    array_key_exists($s ?: 'x', $arr);
    array_key_exists($arr[0], $arr);
    array_key_exists("a$k", $arr);
    array_key_exists($k ?? null, $arr);
    array_key_exists(($k), $arr);
    array_key_exists(...$arr);
    array_key_exists($k, $arr, $m);
    \array_key_exists($m, $arr);
    array_key_exists(array: $arr, key: $m);
}

array_key_exists($top, $arr);
