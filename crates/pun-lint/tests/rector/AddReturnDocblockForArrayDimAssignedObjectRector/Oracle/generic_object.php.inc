<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    public function run(array $keys): array
    {
        $rows = [];
        $rows[] = new \ArrayIterator([1]);

        return $rows;
    }
}
