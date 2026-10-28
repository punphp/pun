<?php

abstract class AbstractGetter
{
    private array $items = [];

    /**
     * @return int[]
     */
    abstract public function getItems(): array;

    /**
     * @return int[]
     */
    public function __get($name): array
    {
        return $this->items;
    }
}
