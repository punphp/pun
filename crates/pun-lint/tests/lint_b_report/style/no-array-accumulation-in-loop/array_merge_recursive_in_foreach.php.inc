<?php

$result = [];
foreach ($items as $item) {
    $result = array_merge_recursive($result, $item);
}
-----
5:15: `array_merge_recursive()` called inside a loop causes O(n²) array copying.
