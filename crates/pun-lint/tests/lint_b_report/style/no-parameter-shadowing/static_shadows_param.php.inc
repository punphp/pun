<?php

function test(int $count): void {
    static $count = 0;

    $count++;
}
-----
3:19: Parameter `$count` is shadowed by a loop, catch, `global`, or `static` variable.
