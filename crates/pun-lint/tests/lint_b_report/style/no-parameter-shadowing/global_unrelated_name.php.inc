<?php

function test(int $value): void {
    global $registry;

    $registry[$value] = true;
}
-----
