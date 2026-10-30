<?php

namespace App;

function run(Missing $callback, $value)
{
    return with($value, $callback);
}
