<?php

final class NotThisReturn
{
    /** @var int[] */
    private array $items = [];

    private self $other;

    public function getItems(): array
    {
        return $this->other->items;
    }

    public function getMore(): array
    {
        $x = 1;

        return $this->items;
    }
}
