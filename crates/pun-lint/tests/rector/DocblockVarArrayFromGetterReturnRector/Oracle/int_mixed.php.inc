<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, mixed>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
