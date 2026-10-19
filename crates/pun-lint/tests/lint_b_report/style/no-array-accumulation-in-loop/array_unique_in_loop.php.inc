<?php

$result = [];
foreach ($items as $item) {
    $result[] = $item;
    $result = array_unique($result);
}
-----
6:15: `array_unique()` called inside a loop reprocesses the entire array every iteration.
