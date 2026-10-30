<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    public function run(array $keys): array
    {
        $rows = [];
        $this->make()[] = 1;
        $rows[] = new Row();

        return $rows;
    }
}
