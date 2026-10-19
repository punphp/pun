<?php

function outer(string $name): void
{
    $fn = function (string $name): void {
        echo $name;
    };
}
-----
