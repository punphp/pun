<?php

function run(int $i, int $j)
{
    $a = ($i + $j) * 1 * 2;
}
-----
<?php

function run(int $i, int $j)
{
    $a = ($i + $j) * 2;
}
