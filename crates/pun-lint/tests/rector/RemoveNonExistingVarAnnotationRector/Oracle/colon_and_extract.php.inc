<?php

function oracle_invalid_var_colon($a)
{
    if ($a):
        /** @var int $nope */
        echo 1;
    endif;

    /** @var int $this */
    echo 2;

    /* plain */
    /** @var int $two */
    echo 3;

    extract(['a' => 1, 'b' => $a]);
    /** @var int $after */
    echo 4;
}

function oracle_invalid_var_const_extract()
{
    extract(['k' => 1, 'l' => 'x']);
    /** @var int $k */
    echo 5;
    echo $k;
}

function oracle_invalid_var_return_static()
{
    /** @var int */
    return Foo::make();
}
-----
<?php

function oracle_invalid_var_colon($a)
{
    if ($a):
        echo 1;
    endif;

    echo 2;

    /* plain */
    /** @var int $two */
    echo 3;

    extract(['a' => 1, 'b' => $a]);
    /** @var int $after */
    echo 4;
}

function oracle_invalid_var_const_extract()
{
    extract(['k' => 1, 'l' => 'x']);
    /** @var int $k */
    echo 5;
    echo $k;
}

function oracle_invalid_var_return_static()
{
    /** @var int */
    return Foo::make();
}
