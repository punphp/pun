<?php
namespace Pun\Oracle\UnusedPrivate;

final class DynamicCallKeepsAll
{
    public function run()
    {
    }
    private function unused()
    {
    }
    private function dynamic($name)
    {
        return $this->$name();
    }
}
