<?php

namespace Oracle\CaseMismatch2;

use ArrayObject;

final class Local
{
}

final class Runner
{
    /** @var ARRAYOBJECT */
    private $c;

    /** @var arrayObject */
    private $d;

    /** @var local */
    private $e;

    /** @var \oracle\casemismatch2\local */
    private $f;
}
-----
<?php

namespace Oracle\CaseMismatch2;

use ArrayObject;

final class Local
{
}

final class Runner
{
    /** @var ArrayObject */
    private $c;

    /** @var ArrayObject */
    private $d;

    /** @var Local */
    private $e;

    /** @var \Oracle\CaseMismatch2\Local */
    private $f;
}
