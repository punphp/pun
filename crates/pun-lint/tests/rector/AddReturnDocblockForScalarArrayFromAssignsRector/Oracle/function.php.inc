<?php

function ints()
{
    $ints = [];
    $ints[] = 1;

    return $ints;
}
-----
<?php

/**
 * @return int[]
 */
function ints()
{
    $ints = [];
    $ints[] = 1;

    return $ints;
}
