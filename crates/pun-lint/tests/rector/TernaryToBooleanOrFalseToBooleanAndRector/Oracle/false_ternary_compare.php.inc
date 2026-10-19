<?php

function f(bool $a, int $b): bool
{
    return $a ? $b === 1 : false;
}

function g(int $b, bool $a): bool
{
    return $b > 1 ? $a : false;
}
-----
<?php

function f(bool $a, int $b): bool
{
    return $a && $b === 1;
}

function g(int $b, bool $a): bool
{
    return $b > 1 && $a;
}
