<?php

foreach ($groups as $group) {
    foreach ($group as $item) {
        $result = array_merge($result, $item);
    }
}
-----
5:19: `array_merge()` called inside a loop causes O(n²) array copying.
