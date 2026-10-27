<?php

namespace Pun\Oracle\UnusedPrivate;

final class DynamicArrayItem
{
    public function run($unknown)
    {
        return [$this, $unknown];
    }

    private function kept()
    {
    }
}
