<?php

namespace Oracle\CaseMismatch;

use Oracle\CaseMismatch\Sub\Helper;
use Oracle\CaseMismatch\Sub\Helper as Aliased;
use Oracle\CaseMismatch\Sub;

final class Local
{
}

final class Runner
{
    /** @var local */
    private $a;

    /** @var \oracle\casemismatch\LOCAL */
    private $b;

    /** @var HELPER */
    private $c;

    /** @var aliased */
    private $d;

    /** @var sub\helper */
    private $e;

    /** @var ?local */
    private $f;

    /** @var local|null */
    private $g;

    /** @var arrayobject */
    private $h;

    /** @var Unknown */
    private $i;

    /**
     * @var local first
     * @var LOCAL second
     */
    private $j;

    public function run()
    {
        /** @var local $x */
        $x = $this->a;

        /** @phpstan-var local $y */
        $y = $this->a;

        return [$x, $y];
    }
}

namespace Oracle\CaseMismatch\Sub;

final class Helper
{
}
-----
<?php

namespace Oracle\CaseMismatch;

use Oracle\CaseMismatch\Sub\Helper;
use Oracle\CaseMismatch\Sub\Helper as Aliased;
use Oracle\CaseMismatch\Sub;

final class Local
{
}

final class Runner
{
    /** @var Local */
    private $a;

    /** @var \Oracle\CaseMismatch\Local */
    private $b;

    /** @var HELPER */
    private $c;

    /** @var aliased */
    private $d;

    /** @var Sub\Helper */
    private $e;

    /** @var ?local */
    private $f;

    /** @var local|null */
    private $g;

    /** @var ArrayObject */
    private $h;

    /** @var Unknown */
    private $i;

    /**
     * @var Local first
     * @var Local second
     */
    private $j;

    public function run()
    {
        /** @var Local $x */
        $x = $this->a;

        /** @phpstan-var local $y */
        $y = $this->a;

        return [$x, $y];
    }
}

namespace Oracle\CaseMismatch\Sub;

final class Helper
{
}
