<?php

namespace ClassTraitCycles;

trait SelfCycle
{
    use selfcycle;
    use MissingSelfTrait;
}

class UsesSelfCycle
{
    use SelfCycle;
}
