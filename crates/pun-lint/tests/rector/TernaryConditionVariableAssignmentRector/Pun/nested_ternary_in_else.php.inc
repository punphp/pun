<?php

function a($v, $w)
{
    $v ? $a = 1 : $a = $w ? 1 : 2;
}
-----
<?php

function a($v, $w)
{
    $a = $v ? 1 : ($w ? 1 : 2);
}
