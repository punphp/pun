<?php

namespace App;

function concat(string $a, $b)
{
    return $a . $b;
}

function param(string $a)
{
    return $a;
}

/** @param string $a */
function docParam($a)
{
    return $a;
}

function builtin(string $a)
{
    return strtoupper($a);
}

function sprintfCall($a)
{
    return sprintf('%s', $a);
}

function mixedStrings(string $a)
{
    if ($a) {
        return 'x';
    }

    return $a;
}

function onlyLiterals()
{
    return 'x';
}

function count2(array $a)
{
    return count($a);
}

function sum(int $a, int $b)
{
    return $a + $b;
}

function division(int $a, int $b)
{
    return $a / $b;
}

function floats(float $a)
{
    return $a * 2.0;
}

function intLiteral()
{
    return -1;
}

function mixedNumbers(int $a)
{
    if ($a) {
        return $a;
    }

    return 1.5;
}

function nullableInt(?int $a)
{
    return $a;
}

final class Names
{
    private string $name = '';

    public function name()
    {
        return $this->name;
    }

    public function className()
    {
        return self::class;
    }

    public function length()
    {
        return strlen($this->name);
    }
}
-----
<?php

namespace App;

function concat(string $a, $b)
{
    return $a . $b;
}

function param(string $a)
{
    return $a;
}

/** @param string $a */
function docParam($a)
{
    return $a;
}

function builtin(string $a)
{
    return strtoupper($a);
}

function sprintfCall($a)
{
    return sprintf('%s', $a);
}

function mixedStrings(string $a)
{
    if ($a) {
        return 'x';
    }

    return $a;
}

function onlyLiterals()
{
    return 'x';
}

function count2(array $a): int
{
    return count($a);
}

function sum(int $a, int $b): int
{
    return $a + $b;
}

function division(int $a, int $b)
{
    return $a / $b;
}

function floats(float $a): float
{
    return $a * 2.0;
}

function intLiteral()
{
    return -1;
}

function mixedNumbers(int $a)
{
    if ($a) {
        return $a;
    }

    return 1.5;
}

function nullableInt(?int $a)
{
    return $a;
}

final class Names
{
    private string $name = '';

    public function name()
    {
        return $this->name;
    }

    public function className()
    {
        return self::class;
    }

    public function length(): int
    {
        return strlen($this->name);
    }
}
