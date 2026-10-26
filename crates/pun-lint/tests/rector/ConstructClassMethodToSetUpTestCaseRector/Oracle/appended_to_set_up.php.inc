<?php

final class W3Test extends \PHPUnit\Framework\TestCase
{
    private $a;

    public function __construct()
    {
        $this->a = 1;

        $this->b = 2;
        parent::__construct();
    }

    protected function setUp(): void
    {
        parent::setUp();
    }
}
-----
<?php

final class W3Test extends \PHPUnit\Framework\TestCase
{
    private $a;

    protected function setUp(): void
    {
        parent::setUp();
        $this->a = 1;
        $this->b = 2;
    }
}
