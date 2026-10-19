<?php

function next(): Closure {
    static $counter = 0;
    return function () use ($counter): int {
        return $counter;
    };
}
