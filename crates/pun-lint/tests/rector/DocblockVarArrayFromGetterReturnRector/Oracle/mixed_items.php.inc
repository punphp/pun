<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return mixed[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
