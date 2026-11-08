<?php

namespace App;

$iterableMember = function (\Countable $x) {
    if (is_iterable($x)) {
        return $x;
    }

    throw new \Exception();
};
