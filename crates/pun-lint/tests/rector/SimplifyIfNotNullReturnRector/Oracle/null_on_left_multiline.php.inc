<?php

function b($x)
{
    $y = 1;

    // before if
    if (null !== $x->foo(
        1
    )) {
        return $x->foo(
            1
        );
    }

    return NULL; // after
}
-----
<?php

function b($x)
{
    $y = 1;
    return $x->foo(
        1
    ); // after
}
