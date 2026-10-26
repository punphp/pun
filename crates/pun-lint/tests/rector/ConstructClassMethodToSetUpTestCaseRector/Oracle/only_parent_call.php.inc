<?php

final class W2Test extends \PHPUnit\Framework\TestCase
{
    private $a;

    public function __construct()
    {
        parent::__construct();
    }
}
-----
<?php

final class W2Test extends \PHPUnit\Framework\TestCase
{
    private $a;

    protected function setUp(): void
    {
    }
}
