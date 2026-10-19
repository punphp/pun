<?php

$result = [];
for ($i = 0; $i < 100; $i++) {
    $result = array_merge($result, getItems($i));
}
-----
5:15: `array_merge()` called inside a loop causes O(n²) array copying.
