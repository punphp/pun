<?php

if (\PHP_VERSION_ID >= 80500) {
    return;
}

function array_last(array $array): mixed
{
    return $array === [] ? null : $array[array_key_last($array)];
}
