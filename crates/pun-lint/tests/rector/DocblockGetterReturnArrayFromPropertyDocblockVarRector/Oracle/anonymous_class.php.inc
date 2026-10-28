<?php

$x = new class {
    /**
     * @var int[]
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
};
