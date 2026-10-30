<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    /**
     * @return array<int, mixed>
     */
    public function run(array $keys): array
    {
        $rows = [];
        $rows[] = new Row();

        return $rows;
    }
}
