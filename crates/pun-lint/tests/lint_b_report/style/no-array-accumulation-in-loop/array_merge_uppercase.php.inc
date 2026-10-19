<?php

$result = [];
foreach ($items as $item) {
    $result = ARRAY_MERGE($result, $item);
}
-----
5:15: `array_merge()` called inside a loop causes O(n²) array copying.
