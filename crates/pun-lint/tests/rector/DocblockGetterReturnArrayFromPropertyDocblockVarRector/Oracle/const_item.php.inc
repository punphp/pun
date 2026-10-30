<?php

final class SomeClass
{
    /**
     * @var array<int, self::FOO>
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
