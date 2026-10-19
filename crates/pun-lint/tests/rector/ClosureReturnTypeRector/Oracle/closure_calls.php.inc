<?php

namespace App;

function local(): int
{
    return 1;
}

$a = function ($f) {
    return $f();
};

$b = function () {
    return local();
};

$c = function () {
    return Sub\other();
};

$d = function () {
    return strlen('a');
};
-----
<?php

namespace App;

function local(): int
{
    return 1;
}

$a = function ($f) {
    return $f();
};

$b = function (): int {
    return local();
};

$c = function () {
    return Sub\other();
};

$d = function (): int {
    return strlen('a');
};
