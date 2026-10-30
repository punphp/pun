<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public static function m(int $i): void
    {
    }
}

function f(A $a): void
{
    $a::m(1);
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class A
{
    public static function m(int $i): void
    {
    }
}

function f(A $a): void
{
    $a::m(1);
}
