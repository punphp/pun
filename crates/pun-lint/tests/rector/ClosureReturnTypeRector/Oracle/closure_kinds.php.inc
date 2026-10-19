<?php

enum Suit
{
    case A;
    case B;
}

final class Keeper
{
    public function f()
    {
        return function () {
            return $this;
        };
    }
}

$e = function () {
    return Suit::A;
};

$f = function ($x) {
    if ($x) {
        return Suit::A;
    }
    return Suit::B;
};

$g = function (array $x) {
    return count($x);
};

$h = function () {
    return 1.5;
};

$i = function ($x) {
    if ($x) {
        return 1.5;
    }
    return 'a';
};

$j = function (array $x) {
    if ($x) {
        return count($x);
    }
    return [1];
};
-----
<?php

enum Suit
{
    case A;
    case B;
}

final class Keeper
{
    public function f()
    {
        return function (): static {
            return $this;
        };
    }
}

$e = function (): \Suit {
    return Suit::A;
};

$f = function ($x): \Suit {
    if ($x) {
        return Suit::A;
    }
    return Suit::B;
};

$g = function (array $x): int {
    return count($x);
};

$h = function (): float {
    return 1.5;
};

$i = function ($x): float|string {
    if ($x) {
        return 1.5;
    }
    return 'a';
};

$j = function (array $x): int|array {
    if ($x) {
        return count($x);
    }
    return [1];
};
