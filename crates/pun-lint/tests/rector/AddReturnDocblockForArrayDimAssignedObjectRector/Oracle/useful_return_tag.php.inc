<?php

namespace App;

final class Row {}

final class SomeClass
{
    private array $rows = [];

    /**
     * @return Row[]
     */
    public function run(array $keys): array
    {
        $rows = [];
        $rows[] = new Row();

        return $rows;
    }
}
