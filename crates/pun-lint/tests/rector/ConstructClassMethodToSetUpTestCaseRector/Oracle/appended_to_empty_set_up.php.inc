<?php

final class W4Test extends \PHPUnit\Framework\TestCase
{
    public function __construct()
    {
        $this->a = 1;
    }

    protected function setUp(): void
    {
    }
}
-----
<?php

final class W4Test extends \PHPUnit\Framework\TestCase
{
    protected function setUp(): void
    {
        $this->a = 1;
    }
}
