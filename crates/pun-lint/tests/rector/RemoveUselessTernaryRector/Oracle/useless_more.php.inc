<?php

function a(array $a, $b)
{
    $x = $a ?: array();
    $y = foo() ?: 0;
    $z = $b->c ? $b->c : null;
}
-----
<?php

function a(array $a, $b)
{
    $x = $a;
    $y = foo() ?: 0;
    $z = $b->c ? $b->c : null;
}
