<?php

namespace ConstructorTraitCycles;

trait SelfCycle
{
    use SelfCycle;
}

class ConstructsSelfCycle
{
    use SelfCycle;

    public function __construct(MissingParameter $value) {}
}
