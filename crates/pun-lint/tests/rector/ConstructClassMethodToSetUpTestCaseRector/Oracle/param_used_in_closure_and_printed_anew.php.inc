<?php

final class W5Test extends \PHPUnit\Framework\TestCase
{
    #[Foo]
    private function __construct(int $a = 1)
    {
        $x = function () use ($a) {};
        parent::__construct($a);
    }
}

final class W6Test extends \PHPUnit\Framework\TestCase
{
    public function __construct(int $a = 1)
    {
        $f = fn () => 1;

        $g = 2;
        parent::__construct($a);
    }
}
-----
<?php

final class W5Test extends \PHPUnit\Framework\TestCase
{
    #[Foo]
    private function __construct(int $a = 1)
    {
        $x = function () use ($a) {};
        parent::__construct($a);
    }
}

final class W6Test extends \PHPUnit\Framework\TestCase
{
    protected function setUp(): void
    {
        $f = fn () => 1;
        $g = 2;
    }
}
