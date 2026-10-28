<?php

namespace App;

final class Row {}

final class StringKeys
{
    /** Gives rows. */
    public function rows(): array
    {
        $rows = [];
        $rows['a'] = new Row();

        return $rows;
    }
}
