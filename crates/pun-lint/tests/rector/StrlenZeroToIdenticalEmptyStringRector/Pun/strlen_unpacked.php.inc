<?php

function a($s, $t)
{
    $x = strlen(...$s) === 0;
    $x = strlen() === 0;
    $x = strlen($s ?: $t) === 0;
    $x = strlen($s = $t) === 0;
}
-----
<?php

function a($s, $t)
{
    $x = strlen(...$s) === 0;
    $x = strlen() === 0;
    $x = (string) ($s ?: $t) === '';
    $x = (string) ($s = $t) === '';
}
