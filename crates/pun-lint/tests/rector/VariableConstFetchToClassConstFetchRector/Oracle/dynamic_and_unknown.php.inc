<?php

namespace Oracle\ConstVarGaps;

final class Sealed
{
    public const NAME = 'n';
}

trait Reads
{
    public function read()
    {
        return $this::NAME;
    }
}

final class Runner
{
    public function run(Sealed $s, Missing $m, string $name)
    {
        echo $s::{'NAME'};
        echo $m::NAME;
        echo $undefined::NAME;
        echo $$name::NAME;
        $made = new Missing();
        echo $made::NAME;
    }
}
