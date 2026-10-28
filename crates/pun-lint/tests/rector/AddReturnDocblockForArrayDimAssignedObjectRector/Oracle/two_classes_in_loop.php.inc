<?php

final class Row {}

final class Other {}

final class SomeClass
{
    public function run(): array
    {
        $rows = [];
        foreach ([1, 2] as $i) {
            $rows[] = new Row();
        }
        foreach ([1, 2] as $i) {
            $rows[] = new Other();
        }

        return $rows;
    }
}
