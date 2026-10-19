<?php

$anonymous = new class {
    private int $value = 1;

    public function __construct()
    {
        $this->value = 2;
    }
};

abstract class AbstractConstructor
{
    private int $value = 1;

    abstract public function __construct();
}

final class DynamicCall
{
    private int $value = 1;

    public function __construct()
    {
        $this->$name();
        $this->value += 2;
        $this->value[] = 2;
        $this->value['key'] = 2;
        $this->value = 3;
    }
}

final class StaticCall
{
    private int $value = 1;

    private int $other = 1;

    public function __construct()
    {
        self::init();
        static::$name();
        Other::init();
    }

    private function init()
    {
        $this->value = 2;
    }
}

final class Test extends \PHPUnit\Framework\TestCase
{
    private int $value = 1;

    public function __construct()
    {
    }

    protected function setUp(): void
    {
        $this->value = 2;
    }
}

final class Autowired
{
    private int $value = 1;

    private int $other = 1;

    public function __construct()
    {
    }

    /**
     * @required
     */
    public function inject()
    {
        $this->value = 2;
    }

    /**
     * @requiredness
     */
    public function notTagged()
    {
        $this->other = 2;
    }
}

final class ColonIf
{
    private int $value = 1;

    private int $other = 1;

    private int $third = 1;

    public function __construct($x)
    {
        if ($x):
            $this->value = 2;
        else:
            $this->value = 3;
        endif;
        if ($x):
            $this->other = 2;
        elseif ($x > 1):
            $this->other = 3;
        else:
            $this->other = 4;
        endif;
        if ($x):
            $this->third = 2;
        endif;
    }
}

final class BlockIf
{
    private int $value = 1;

    private int $other = 1;

    public function __construct($x)
    {
        if ($x)
            $this->value = 2;
        else {
            ;
            $this->value = 3;
        }
        if ($x) {
            $this->other = 2;
        } else {
            foo();
            $this->other = 3;
        }
    }
}

final class NestedKeys
{
    private array $value = [];

    public function __construct()
    {
        $this->value[1][] = 2;
        $this->value = [];
    }
}
-----
<?php

$anonymous = new class {
    private int $value;

    public function __construct()
    {
        $this->value = 2;
    }
};

abstract class AbstractConstructor
{
    private int $value = 1;

    abstract public function __construct();
}

final class DynamicCall
{
    private int $value = 1;

    public function __construct()
    {
        $this->$name();
        $this->value += 2;
        $this->value[] = 2;
        $this->value['key'] = 2;
        $this->value = 3;
    }
}

final class StaticCall
{
    private int $value;

    private int $other = 1;

    public function __construct()
    {
        self::init();
        static::$name();
        Other::init();
    }

    private function init()
    {
        $this->value = 2;
    }
}

final class Test extends \PHPUnit\Framework\TestCase
{
    private int $value;

    public function __construct()
    {
    }

    protected function setUp(): void
    {
        $this->value = 2;
    }
}

final class Autowired
{
    private int $value;

    private int $other = 1;

    public function __construct()
    {
    }

    /**
     * @required
     */
    public function inject()
    {
        $this->value = 2;
    }

    /**
     * @requiredness
     */
    public function notTagged()
    {
        $this->other = 2;
    }
}

final class ColonIf
{
    private int $value;

    private int $other = 1;

    private int $third = 1;

    public function __construct($x)
    {
        if ($x):
            $this->value = 2;
        else:
            $this->value = 3;
        endif;
        if ($x):
            $this->other = 2;
        elseif ($x > 1):
            $this->other = 3;
        else:
            $this->other = 4;
        endif;
        if ($x):
            $this->third = 2;
        endif;
    }
}

final class BlockIf
{
    private int $value;

    private int $other;

    public function __construct($x)
    {
        if ($x)
            $this->value = 2;
        else {
            ;
            $this->value = 3;
        }
        if ($x) {
            $this->other = 2;
        } else {
            foo();
            $this->other = 3;
        }
    }
}

final class NestedKeys
{
    private array $value = [];

    public function __construct()
    {
        $this->value[1][] = 2;
        $this->value = [];
    }
}
