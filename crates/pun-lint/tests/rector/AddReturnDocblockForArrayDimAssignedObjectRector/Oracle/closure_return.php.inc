<?php

final class Row {}

final class Other {}

final class SomeClass
{
    public function run(): array
    {
        $rows = [];
        $rows[] = new Row();
        $f = function () {
            $x = 1;

            return $x;
        };

        return $rows;
    }
}
