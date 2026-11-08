<?php

final class NotArrayAssigned
{
    private array $items;

    private array $other;

    public function __construct(array $items)
    {
        $this->items = $items;
    }
}
