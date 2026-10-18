<?php

function multipleBooleanNotKeepParentheses($a, $b)
{
    $r = !!!($a && $b);
    $r = !!$a instanceof Foo;
    $r = !!($a instanceof Foo);
    $r = !!($a = $b) && $b;
    $r = !!!($a = $b) && $b;
}
-----
<?php

function multipleBooleanNotKeepParentheses($a, $b)
{
    $r = !($a && $b);
    $r = (bool) ($a instanceof Foo);
    $r = (bool) ($a instanceof Foo);
    $r = (bool) ($a = $b) && $b;
    $r = !($a = $b) && $b;
}
