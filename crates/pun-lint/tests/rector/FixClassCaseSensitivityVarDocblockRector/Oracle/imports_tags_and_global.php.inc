<?php

namespace Oracle\CaseMismatchGaps;

use Random;

final class Local
{
}

final class Runner
{
    /** @var random\randomizer */
    private $a;

    /** @var stdclass */
    private $b;

    /** @psalm-var local */
    private $c;

    /** @varlocal */
    private $d;

    /** @var local @var stdclass */
    private $e;

    /** @var nick */
    private $f;
}

class_alias(Local::class, 'Oracle\CaseMismatchGaps\Nick');
-----
<?php

namespace Oracle\CaseMismatchGaps;

use Random;

final class Local
{
}

final class Runner
{
    /** @var Random\Randomizer */
    private $a;

    /** @var stdClass */
    private $b;

    /** @psalm-var local */
    private $c;

    /** @varlocal */
    private $d;

    /** @var Local @var stdclass */
    private $e;

    /** @var nick */
    private $f;
}

class_alias(Local::class, 'Oracle\CaseMismatchGaps\Nick');
