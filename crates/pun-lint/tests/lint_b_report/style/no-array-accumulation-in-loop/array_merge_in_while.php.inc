<?php

$result = [];
while ($item = getNext()) {
    $result = array_merge($result, $item);
}
-----
5:15: `array_merge()` called inside a loop causes O(n²) array copying.
