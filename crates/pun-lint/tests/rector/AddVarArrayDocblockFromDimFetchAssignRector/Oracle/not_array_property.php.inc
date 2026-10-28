<?php

final class NotArrayProperty
{
    private $items = [];

    private string $name = '';

    public function add(): void
    {
        $this->items[] = 5;
    }
}
