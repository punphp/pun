<?php

function handle(Exception $e): void
{
    try {
        doSomething();
    } catch (RuntimeException $e) {
        log($e);
    }
}
-----
3:27: Parameter `$e` is shadowed by a loop, catch, `global`, or `static` variable.
