<?php

final class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function test()
    {
        $a = $this->other->createMock(X::class);
        $a = 1;
        $b = $this->createMock(X::class);
        $b = 1;
        return [$a, $b];
    }
}
-----
<?php

final class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function test()
    {
        $a = $this->other->createMock(X::class);
        $a = 1;
        $b = 1;
        return [$a, $b];
    }
}
