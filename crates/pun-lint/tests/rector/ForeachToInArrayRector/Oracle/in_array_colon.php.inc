<?php

function colon(array $items, $x)
{
    foreach ($items as $item):
        if ($item === $x) {
            return true;
        }
    endforeach;
    return false;
}
-----
<?php

function colon(array $items, $x)
{
    return in_array($x, $items, true);
}
