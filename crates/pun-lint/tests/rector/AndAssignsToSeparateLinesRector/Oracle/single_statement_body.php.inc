<?php

function andAssignsSingleStatementBody($a, $b)
{
    if ($a) $a = 1 and $b = 2;
    foreach ([1] as $a) $a = 1 and $b = 2;
}
-----
<?php

function andAssignsSingleStatementBody($a, $b)
{
    if ($a) {
        $a = 1;
        $b = 2;
    }
    foreach ([1] as $a) {
        $a = 1;
        $b = 2;
    }
}
