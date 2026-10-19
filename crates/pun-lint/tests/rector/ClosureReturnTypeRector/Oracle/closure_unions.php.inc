<?php

enum Level
{
    case Low;
    case High;
}

$a = function (array $x, $y) {
    return $y ? count($x) : [1];
};

$b = function ($y) {
    return $y ? Level::Low : 1;
};

$c = function ($y) {
    return $y ? 1.5 : 'a';
};
-----
<?php

enum Level
{
    case Low;
    case High;
}

$a = function (array $x, $y): int|array {
    return $y ? count($x) : [1];
};

$b = function ($y): \Level|int {
    return $y ? Level::Low : 1;
};

$c = function ($y): float|string {
    return $y ? 1.5 : 'a';
};
