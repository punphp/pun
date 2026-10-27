<?php

namespace Pun\Oracle\UnusedPrivate;

final class DynamicPartialCall
{
    public function run($name)
    {
        return $this->$name(...);
    }

    private function kept()
    {
    }
}
