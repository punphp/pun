<?php

final class SomeClass
{
    public function run(array $items): void
    {
        usort($items, fn (int $a, string $b) => 0);
    }
}
