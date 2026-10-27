<?php

namespace Pun\Oracle\UnusedPrivate;

final class SelfDynamicCall
{
    public function run()
    {
    }

    private function viaPartial($name)
    {
        return $this->$name(...);
    }

    private function viaBraced($name)
    {
        return $this->{$name}();
    }

    private function unused()
    {
    }
}
