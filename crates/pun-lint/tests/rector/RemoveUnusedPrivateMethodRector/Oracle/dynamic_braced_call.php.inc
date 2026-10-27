<?php

namespace Pun\Oracle\UnusedPrivate;

final class DynamicBracedCall
{
    private function kept()
    {
    }

    private function run($name)
    {
        return $this->{$name}();
    }
}
