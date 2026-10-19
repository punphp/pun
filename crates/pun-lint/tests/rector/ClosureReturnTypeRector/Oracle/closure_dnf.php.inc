<?php

interface A
{
}

interface B
{
}

$f = function (bool $b, A&B $x) {
    return $b ? $x : null;
};
-----
<?php

interface A
{
}

interface B
{
}

$f = function (bool $b, A&B $x): (\A&\B)|null {
    return $b ? $x : null;
};
