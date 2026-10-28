<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_walk($items, function (int $a, int $b, int $c) {
        });
    }
}
