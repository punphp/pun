<?php

function a($s, $t, $u)
{
    $x = strlen($s) === 1;
    $x = 0 > strlen($s);
    $x = strlen($s) < 0;
    $x = strlen(string: $s) === 0;
    $x = strlen($s . $t) === 0;
    $x = strlen($s ?? $t) === 0;
}
-----
<?php

function a($s, $t, $u)
{
    $x = strlen($s) === 1;
    $x = 0 > strlen($s);
    $x = strlen($s) < 0;
    $x = (string) $s === '';
    $x = $s . $t === '';
    $x = (string) ($s ?? $t) === '';
}
