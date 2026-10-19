<?php

namespace App;

interface Plain
{
    public function run();
}

final class ImplementsPlain implements Plain
{
    public function __construct($unused)
    {
    }

    public function run()
    {
    }
}

final class MiddleUnused
{
    private $b;

    public function __construct($a, $b)
    {
        $this->b = $b;
    }
}

#[\Attribute]
final class MarkerWithoutDoc
{
    public function __construct(string $marker)
    {
    }
}

final class NoParams
{
    public function __construct()
    {
    }
}

final class Documented
{
    /**
     * @param string $first
     * @param int $second
     */
    public function __construct($first, $second)
    {
        echo $first;
    }
}

$object = new class (1) {
    public function __construct($value)
    {
    }
};

final class WithoutConstructor
{
    public function run($a)
    {
    }
}
-----
<?php

namespace App;

interface Plain
{
    public function run();
}

final class ImplementsPlain implements Plain
{
    public function __construct()
    {
    }

    public function run()
    {
    }
}

final class MiddleUnused
{
    private $b;

    public function __construct($a, $b)
    {
        $this->b = $b;
    }
}

#[\Attribute]
final class MarkerWithoutDoc
{
    public function __construct()
    {
    }
}

final class NoParams
{
    public function __construct()
    {
    }
}

final class Documented
{
    /**
     * @param string $first
     */
    public function __construct($first)
    {
        echo $first;
    }
}

$object = new class (1) {
    public function __construct()
    {
    }
};

final class WithoutConstructor
{
    public function run($a)
    {
    }
}
