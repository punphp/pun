<?php

namespace TraitCycles;

trait SelfCycle
{
    use selfcycle;
    use MissingSelfTrait;
}

enum UsesSelfCycle
{
    use SelfCycle;
}

trait First
{
    use Second;
    use MissingFirstTrait;
}

trait Second
{
    use First;
    use MissingSecondTrait;
}

enum UsesMutualCycle
{
    use First;
}
