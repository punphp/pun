<?php

function process(string $key, array $items): void
{
    foreach ($items as $key => $items) {
        echo $key;
    }
}
-----
3:25: Parameter `$key` is shadowed by a loop, catch, `global`, or `static` variable.
3:37: Parameter `$items` is shadowed by a loop, catch, `global`, or `static` variable.
