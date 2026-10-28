<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    public function run(array $keys): array
    {
        $rows = [new Row()];
        $rows[] = new Row();

        return $rows;
    }
}
