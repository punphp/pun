<?php

$chunks = [];
foreach ($items as $item) {
    $chunks[] = $item;
}
$result = array_merge(...$chunks);
-----
