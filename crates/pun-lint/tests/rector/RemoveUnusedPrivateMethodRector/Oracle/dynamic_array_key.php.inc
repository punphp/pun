<?php

namespace Pun\Oracle\UnusedPrivate;

final class DynamicArrayKey
{
    public function run($k)
    {
        return [$k => $this, 'x'];
    }

    private function kept()
    {
    }
}
