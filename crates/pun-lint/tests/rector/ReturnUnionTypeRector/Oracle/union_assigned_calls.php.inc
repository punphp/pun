<?php

namespace App;

function g(): int
{
    return 1;
}

function a(bool $b)
{
    $f = 'strlen';
    $x = $f('a');
    $y = g();
    $z = Sub\h();
    if ($b) {
        return $y;
    }
    return 'none';
}
-----
<?php

namespace App;

function g(): int
{
    return 1;
}

function a(bool $b): int|string
{
    $f = 'strlen';
    $x = $f('a');
    $y = g();
    $z = Sub\h();
    if ($b) {
        return $y;
    }
    return 'none';
}
