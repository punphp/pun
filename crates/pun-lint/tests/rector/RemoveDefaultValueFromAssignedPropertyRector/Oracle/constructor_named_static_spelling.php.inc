<?php

namespace ConstructorNameSpelling;

final class NameSpelling
{
    private static $caseMismatch;
    private static $relative;
    private int $value = 0;

    public function __construct()
    {
        nAmEsPeLlInG::$caseMismatch = 1;
        namespace\NameSpelling::$relative = 2;
        nAmEsPeLlInG::initialize();
    }

    private function initialize(): void
    {
        $this->value = 3;
    }
}
-----
<?php

namespace ConstructorNameSpelling;

final class NameSpelling
{
    private static $caseMismatch;
    private static $relative;
    private int $value;

    public function __construct()
    {
        nAmEsPeLlInG::$caseMismatch = 1;
        namespace\NameSpelling::$relative = 2;
        nAmEsPeLlInG::initialize();
    }

    private function initialize(): void
    {
        $this->value = 3;
    }
}
