<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (mixed $item) => $item, $items);
    }
}
