<?php

final class W1Test extends \PHPUnit\Framework\TestCase
{
    private $a;

    #[Foo]
    final public function __construct()
    {
        // first
        $this->a = 1;

        Parent::__CONSTRUCT();
        // last
    }
}
-----
<?php

final class W1Test extends \PHPUnit\Framework\TestCase
{
    private $a;

    #[Foo]
    final protected function setUp(): void
    {
        // first
        $this->a = 1;
        // last
    }
}
