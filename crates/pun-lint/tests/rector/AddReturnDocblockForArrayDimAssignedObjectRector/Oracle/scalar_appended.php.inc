<?php

final class Row {}

final class Other {}

final class SomeClass
{
    public function run(): array
    {
        $rows = [];
        $rows[] = new Row();
        $rows[] = 1;

        return $rows;
    }
}
