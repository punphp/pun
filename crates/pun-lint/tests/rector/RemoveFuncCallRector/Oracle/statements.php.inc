<?php

namespace N;

use function A\dump as var_dump;

function run($x)
{
    var_dump($x);
    \var_dump($x);
    VAR_DUMP($x);
    // comment
    \A\var_dump($x);
    $var_dump($x);
    if ($x) var_dump($x);
    if ($x) {
        var_dump($x);
    }
    $y = var_dump($x);
    var_dump(...);
    return $y;
}
-----
<?php

namespace N;

use function A\dump as var_dump;

function run($x)
{
    var_dump($x);
    VAR_DUMP($x);
    // comment
    \A\var_dump($x);
    if ($x) var_dump($x);
    if ($x) {
        var_dump($x);
    }
    $y = var_dump($x);
    var_dump(...);
    return $y;
}
