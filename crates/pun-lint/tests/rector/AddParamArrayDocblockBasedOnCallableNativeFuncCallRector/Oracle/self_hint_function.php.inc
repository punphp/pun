<?php

function run(array $items): void
{
    array_map(fn (self $item) => $item, $items);
}
