<?php

$fn = function (array $items): void {
    foreach ($items as $key => $items) {
        echo $key;
    }
};
-----
3:23: Parameter `$items` is shadowed by a loop, catch, `global`, or `static` variable.
