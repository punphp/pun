<?php

namespace App;

function byReference($name, $other)
{
    $name = &$other;
    return sprintf('%s', $name);
}

function defaultFirst($code)
{
    return match ($code) {
        default => 'x',
        1, 2 => sprintf('%s', $code),
    };
}
-----
<?php

namespace App;

function byReference(string $name, $other)
{
    $name = &$other;
    return sprintf('%s', $name);
}

function defaultFirst(string $code)
{
    return match ($code) {
        default => 'x',
        1, 2 => sprintf('%s', $code),
    };
}
