<?php

final class SomeClass
{
    /**
     * @var int[]|string[]
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
