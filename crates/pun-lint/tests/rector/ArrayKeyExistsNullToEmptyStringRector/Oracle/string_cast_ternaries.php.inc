<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function ternaries($k, mixed $x, ?string $s, $arr)
{
    array_key_exists((string) $k ? $x : null, $arr);
    array_key_exists((string) $k ? 'a' : null, $arr);
    array_key_exists((int) $k ? $x : null, $arr);
    array_key_exists((string) $k ?: null, $arr);
    array_key_exists($k ? 'a' : null, $arr);
    array_key_exists($k ? 'a' : $s, $arr);
    array_key_exists($pl->$k, $arr);
    array_key_exists(Missing::$$k, $arr);
}

function unreachable($k, $arr)
{
    return;
    array_key_exists($k, $arr);
}

function spread(array $a, array $b)
{
    array_key_exists(...$a, ...$b);
}

function named_explicit(mixed $x, $arr)
{
    if ($x !== null) {
        array_key_exists($x, $arr);
    }
}

$g = gk();
if ($g !== null) {
    array_key_exists($g, $arr);
}

function gk(): mixed
{
    return 1;
}
-----
<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function ternaries($k, mixed $x, ?string $s, $arr)
{
    array_key_exists((string) $k ? $x : null, $arr);
    array_key_exists((string) $k ? 'a' : '', $arr);
    array_key_exists((int) $k ? $x : '', $arr);
    array_key_exists((string) $k ?: '', $arr);
    array_key_exists($k ? 'a' : '', $arr);
    array_key_exists($k ? 'a' : (string) $s, $arr);
    array_key_exists((string) $pl->$k, $arr);
    array_key_exists((string) Missing::$$k, $arr);
}

function unreachable($k, $arr)
{
    return;
    array_key_exists((string) $k, $arr);
}

function spread(array $a, array $b)
{
    array_key_exists(...$a, ...$b);
}

function named_explicit(mixed $x, $arr)
{
    if ($x !== null) {
        array_key_exists((string) $x, $arr);
    }
}

$g = gk();
if ($g !== null) {
    array_key_exists((string) $g, $arr);
}

function gk(): mixed
{
    return 1;
}
