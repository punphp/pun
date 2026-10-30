<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn ($item) => $item, $items);
    }
}
