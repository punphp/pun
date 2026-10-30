<?php

final class Keyed
{
    private array $items = [];

    public function add(string $key): void
    {
        $this->items[$key] = 'value';
    }
}
