<?php

namespace DebugTypes;

use function PHPStan\dumpType as dtype;
use function PHPStan\dumpPhpDocType as ptype;

/** @param int<0, 100> $n */
function ranged(int $n): void
{
    dtype($n);
    ptype($n);
}

/**
 * @template T
 * @param T $value
 */
function generic($value): void
{
    dtype($value);
    ptype($value);
    dtype([$value]);
    ptype([$value]);
}

class C
{
    public function inspect(): void
    {
        dtype($this);
        ptype($this);
        dtype([$this]);
        ptype([$this]);
    }
}

dtype('');
ptype('');
dtype("\t");
ptype("\t");
dtype(['Foo\\Bar' => 'Foo\\Bar']);
ptype(['Foo\\Bar' => 'Foo\\Bar']);
ptype(['Let\'s go' => "Let's go"]);
ptype(["\0" => 'NUL', 'NUL' => "\0"]);
ptype(["\t" => 'HT', 'HT' => "\t"]);
ptype(['' => '', 'foo?' => true, 'shall-we-dance' => false, '3.14' => 3.14]);
ptype([true => true, false => false]);
ptype(['nested' => ['a', 2]]);
dtype('123456789012345678901234567890');
dtype([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

function alternatives(bool $flag): void
{
    dtype($flag ? 1 : 'a');
    ptype($flag ? 1 : 'a');
}
