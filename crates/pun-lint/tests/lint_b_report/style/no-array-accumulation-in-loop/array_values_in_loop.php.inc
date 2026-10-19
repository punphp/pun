<?php

$result = [];
foreach ($items as $item) {
    $result[] = $item;
    $result = array_values($result);
}
-----
6:15: `array_values()` called inside a loop reprocesses the entire array every iteration.
