<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    public function run(array $keys): iterable
    {
        $rows = [];
        $rows[] = new Row();

        return $rows;
    }
}
