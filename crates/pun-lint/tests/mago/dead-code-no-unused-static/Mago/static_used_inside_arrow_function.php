<?php

function next(): Closure {
    static $counter = 0;
    return fn() => $counter;
}
