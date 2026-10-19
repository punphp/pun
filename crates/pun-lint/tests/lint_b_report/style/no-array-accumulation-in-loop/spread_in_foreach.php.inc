<?php

$result = [];
foreach ($items as $item) {
    $result = [...$result, ...$item];
}
-----
5:5: Array spread in reassignment inside a loop causes O(n²) array copying.
