<?php

final class Row {}

final class Other {}

final class SomeClass
{
    public function run(): array
    {
        $rows = [new Row()];
        $rows[] = new Row();

        return $rows;
    }
}
