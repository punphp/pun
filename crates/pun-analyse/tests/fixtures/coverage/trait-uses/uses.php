<?php

namespace RepeatedTrait;

trait Source
{
    public function run(): MissingReturn
    {
        throw new \Exception();
    }
}

class UsesTwice
{
    use Source;
    use Source { run as runAgain; }
}

class SeparateAliases
{
    use Source { run as firstRun; }
    use Source { run as secondRun; }
}

class RepeatedPlain
{
    use Source;
    use Source;
}

class UsesOnce
{
    use Source;
}

trait Left
{
    use Source;
}

trait Right
{
    use Source;
}

class Diamond
{
    use Left, Right;
}
