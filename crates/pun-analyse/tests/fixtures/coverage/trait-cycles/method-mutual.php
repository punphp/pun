<?php

namespace MethodTraitCycles;

trait First
{
    use Second;

    public function run(): MissingReturn
    {
        throw new \Exception();
    }
}

trait Second
{
    use First;
}

class UsesMutualCycle
{
    use First;
}
