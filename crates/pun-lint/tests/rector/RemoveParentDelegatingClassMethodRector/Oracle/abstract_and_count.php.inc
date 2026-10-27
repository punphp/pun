<?php

namespace Pun\Oracle\UselessOverride;

interface Runs
{
    public function run($value);
}

abstract class AbstractRunner implements Runs
{
    abstract public function run($value);
}

class CountBase
{
    public function go($a, $b = 1)
    {
        return $a + $b;
    }
}

final class FewerArguments extends CountBase
{
    public function go($a, $b = 1)
    {
        return parent::go($a);
    }
}
