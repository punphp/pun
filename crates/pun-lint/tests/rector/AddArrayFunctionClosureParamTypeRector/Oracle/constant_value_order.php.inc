<?php

namespace Probe;

/**
 * @param int|null $count
 * @param float $ratio
 */
function orders(string $name, $count, $ratio, bool $flag)
{
    array_filter([1, 'a', null], fn ($a) => $a !== null);
    array_filter(['a', 1], fn ($b) => $b !== null);
    array_filter([null, 'a', 1.5, true], fn ($c) => $c !== null);
    array_filter(['x' => $count, 'y' => $name], fn ($d) => $d !== null);
    array_filter([$ratio, $count, $name], fn ($e) => $e !== null);
    array_filter([$flag, 1], fn ($f) => $f !== null);
}
-----
<?php

namespace Probe;

/**
 * @param int|null $count
 * @param float $ratio
 */
function orders(string $name, $count, $ratio, bool $flag)
{
    array_filter([1, 'a', null], fn (int|string|null $a) => $a !== null);
    array_filter(['a', 1], fn (string|int $b) => $b !== null);
    array_filter([null, 'a', 1.5, true], fn (null|string|float|true $c) => $c !== null);
    array_filter(['x' => $count, 'y' => $name], fn (int|string|null $d) => $d !== null);
    array_filter([$ratio, $count, $name], fn (float|int|string|null $e) => $e !== null);
    array_filter([$flag, 1], fn (bool|int $f) => $f !== null);
}
