<?php

final class Row {}

final class SomeClass
{
    public function run(): array
    {
        $rows = [];
        $other = [];
        $other[] = 1;
        $rows[] = new Row();

        return $rows;
    }
}
