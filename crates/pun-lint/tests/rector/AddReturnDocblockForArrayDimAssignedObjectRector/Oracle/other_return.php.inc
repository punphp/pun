<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    public function run(array $keys): array
    {
        $rows = [];
        $rows[] = new Row();
        $other = [];
        if ($keys) {
            return $other;
        }

        return $rows;
    }
}
