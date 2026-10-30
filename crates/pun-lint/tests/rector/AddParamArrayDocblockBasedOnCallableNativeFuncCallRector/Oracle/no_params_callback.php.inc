<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn () => 1, $items);
    }
}
