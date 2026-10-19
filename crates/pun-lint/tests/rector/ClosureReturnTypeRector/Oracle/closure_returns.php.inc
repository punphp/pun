<?php

$a = function ($x) {
    if ($x) {
        return;
    }
    return 1;
};

$b = function ($x) {
    return $x;
};

$c = function ($x) {
    if ($x) {
        return 1;
    }
};

$d = function () use ($a) {
    return 1;
};

$e = function () {
    return new class {};
};

$f = function () {
    return null;
};
-----
<?php

$a = function ($x) {
    if ($x) {
        return;
    }
    return 1;
};

$b = function ($x) {
    return $x;
};

$c = function ($x) {
    if ($x) {
        return 1;
    }
};

$d = function () use ($a): int {
    return 1;
};

$e = function (): object {
    return new class {};
};

$f = function (): null {
    return null;
};
