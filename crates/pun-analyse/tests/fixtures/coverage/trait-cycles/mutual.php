<?php

namespace ClassTraitCycles;

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

class UsesMutualCycle
{
    use First;
}
