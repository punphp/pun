<?php

namespace ConstructorTraitCycles;

trait First
{
    use Second;
}

trait Second
{
    use First;
}

class ConstructsMutualCycle
{
    use First;

    public function __construct(MissingParameter $value) {}
}
