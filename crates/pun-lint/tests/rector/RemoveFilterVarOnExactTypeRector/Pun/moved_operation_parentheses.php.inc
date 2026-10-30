<?php

function run(int $i, bool $b)
{
    $a = filter_var($i + 1, FILTER_VALIDATE_INT) * 2;
    $a = -filter_var($i - 1, FILTER_VALIDATE_INT);
    $a = filter_var($b ? 1 : 2, FILTER_VALIDATE_INT) + 1;
}
-----
<?php

function run(int $i, bool $b)
{
    $a = ($i + 1) * 2;
    $a = -($i - 1);
    $a = ($b ? 1 : 2) + 1;
}
