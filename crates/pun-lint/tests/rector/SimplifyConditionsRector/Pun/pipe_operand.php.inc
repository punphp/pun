<?php

function run($a, $b)
{
    return !($a |> trim(...) === $b);
}
