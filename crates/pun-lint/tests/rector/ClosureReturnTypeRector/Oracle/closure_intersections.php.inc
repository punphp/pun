<?php

namespace App;

final class Invokable
{
}

$callableIntersection = function (callable $x) {
    if ($x instanceof Invokable) {
        return $x;
    }

    return new Invokable();
};

