<?php

namespace MethodTraitCycles;

trait SelfCycle
{
    use selfcycle;

    public function run(): MissingReturn
    {
        throw new \Exception();
    }
}

class UsesSelfCycle
{
    use SelfCycle;
    use SelfCycle { run as runAgain; }
}
