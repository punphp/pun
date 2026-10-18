<?php

function constantBooleanNotOracle($a)
{
    $r = !FALSE;
    $r = !\true;
    $r = !(false);
    $r = !null;
    $r = !$a;
    $r = -true;
    $r = !Foo\true;
}
-----
<?php

function constantBooleanNotOracle($a)
{
    $r = true;
    $r = false;
    $r = true;
    $r = !null;
    $r = !$a;
    $r = -true;
    $r = !Foo\true;
}
