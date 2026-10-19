<?php

function process(string $key, array $items): void
{
    foreach ($items as $key => $value) {
        echo $value;
    }
}
-----
3:25: Parameter `$key` is shadowed by a loop, catch, `global`, or `static` variable.
