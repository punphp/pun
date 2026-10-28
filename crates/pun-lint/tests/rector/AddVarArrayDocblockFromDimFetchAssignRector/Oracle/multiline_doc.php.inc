<?php

final class MultilineDoc
{
    /**
     * @var array<
     *     int
     * >
     */
    private array $items = [];

    public function add(): void
    {
        $this->items[] = 5;
    }
}
