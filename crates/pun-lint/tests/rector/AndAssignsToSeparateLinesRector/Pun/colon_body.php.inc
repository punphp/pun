<?php

function andAssignsColonBody($a, $b)
{
    if ($a):
        $a = 1 and $b = 2;
    endif;
}
-----
<?php

function andAssignsColonBody($a, $b)
{
    if ($a):
        $a = 1;
        $b = 2;
    endif;
}
