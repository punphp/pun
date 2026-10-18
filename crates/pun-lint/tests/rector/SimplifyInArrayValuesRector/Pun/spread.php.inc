<?php

function inArrayValuesSpread($needle, $haystack)
{
    $r = in_array($needle, ...array_values($haystack));
    $r = in_array($needle, array_values(...$haystack));
    $r = in_array($needle, array_values());
}
