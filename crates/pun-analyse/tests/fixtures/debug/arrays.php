<?php

namespace DebugArrays;

use function PHPStan\dumpType as dtype;
use function PHPStan\dumpPhpDocType as ptype;

dtype(['z' => 1, 'a' => 2, 'm' => 3]);
ptype(['z' => 1, 'a' => 2, 'z' => 3]);
ptype(array('z' => array('y' => 1, 'b' => 2), 'a' => 3));
ptype((['z' => ['y' => 1, 'b' => 2], 'a' => 3]));
ptype([true => 'one', false => 'zero', '1' => 'replaced', '01' => 'string']);
ptype([null => 'null', '' => 'replaced', '-1' => 'negative', '+1' => 'string', '-0' => 'string']);
ptype([2 => 'two', 'zero', 0 => 'zero-key', 'four']);
ptype(['z' => 1, ...['b' => 2, 'a' => 3], 'b' => 4]);
ptype([10 => 'ten', ...[99 => 'x', 3 => 'y'], 'z']);
ptype([...[true, false], ...[], true]);
ptype([]);
ptype([1 => ['z' => 1, 'a' => 2], 0 => ['b' => false, 'a' => true]]);

/** @param array<string, int> $unknown */
function unknownArray(array $unknown): void
{
    dtype($unknown);
    ptype($unknown);
    dtype([...$unknown]);
}

function unknownKey(string $key, int $value): void
{
    dtype([$key => $value]);
    ptype([$key => $value]);
    ptype([...[$key => $value]]);
}

/** @param list<int> $list */
function listFallback(array $list): void
{
    dtype($list);
    ptype($list);
}

function knownKey(): void
{
    $key = '9';
    ptype(['z' => 0, $key => true, 'a' => 2]);
}

ptype([1.9 => 'float', 1 => 'replaced', 0.0 => 'zero']);
ptype(['9223372036854775808' => 'string', '-9223372036854775808' => 'int', '0' => 'zero']);

function nonliteralUnpack(): void
{
    $values = ['a' => 1, 'z' => 2];
    ptype([...$values]);
}

ptype([-5 => 'negative', 'next']);
ptype([9223372036854775807 => 'max']);
