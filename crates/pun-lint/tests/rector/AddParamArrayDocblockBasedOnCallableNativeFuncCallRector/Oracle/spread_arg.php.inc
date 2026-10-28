<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (int $item) => $item, ...$items);
    }
}
