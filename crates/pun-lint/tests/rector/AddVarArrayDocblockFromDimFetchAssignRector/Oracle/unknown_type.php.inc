<?php

final class UnknownType
{
    private array $items = [];

    public function add(): void
    {
        $this->items[] = unknown_function();
    }
}
