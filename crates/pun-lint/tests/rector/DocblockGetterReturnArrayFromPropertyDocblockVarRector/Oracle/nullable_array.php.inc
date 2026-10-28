<?php

final class SomeClass
{
    /**
     * @var ?array<int, string>
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
