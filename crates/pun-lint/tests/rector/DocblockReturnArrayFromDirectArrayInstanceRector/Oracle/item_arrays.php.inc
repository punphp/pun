<?php

namespace App;

final class Item {}

final class ItemArrays
{
    /** @var Item[] */
    private array $items = [];

    public function run()
    {
        return [$this->items];
    }
}
