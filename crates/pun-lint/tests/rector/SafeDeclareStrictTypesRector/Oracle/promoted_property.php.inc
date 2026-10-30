<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct(public int $p)
    {
    }
}

function f(A $a): void
{
    $a->p = 1;
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct(public int $p)
    {
    }
}

function f(A $a): void
{
    $a->p = 1;
}
