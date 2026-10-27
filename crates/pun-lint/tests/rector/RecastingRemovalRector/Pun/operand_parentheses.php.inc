<?php

function run(int $i)
{
    $x = (int) ($i + 1) * 2;
}
-----
<?php

function run(int $i)
{
    $x = ($i + 1) * 2;
}
