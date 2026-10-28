<?php

final class MultilineDoc
{
    /**
     * @var array<
     *     int
     * >
     */
    private array $items;

    /**
     * @param int[] $items
     */
    public function __construct(array $items)
    {
        $this->items = $items;
    }
}
