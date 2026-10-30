<?php

function run($x)
{
    try {
        $x();
    } catch (\Throwable $e) {
        throw new \ErrorException('m', 1, $e);
    }
}
