<?php

function global_call($a)
{
    if ($a) {
        return is_int($a);
    }
    return $a === 1;
}
-----
<?php

function global_call($a): bool
{
    if ($a) {
        return is_int($a);
    }
    return $a === 1;
}
