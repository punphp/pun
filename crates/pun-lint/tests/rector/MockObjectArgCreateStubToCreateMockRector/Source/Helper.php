<?php

namespace Pun\Ns;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;

class Helper
{
    public function take(MockObject $m): void {}
    public static function takeStatic(MockObject $m): void {}
    /** @param MockObject $m */
    public function docOnly($m): void {}
    public function nullable(?MockObject $m): void {}
    public function untyped($m): void {}
    public function stub(Stub $m): void {}
    public function variadic(MockObject ...$m): void {}
    public function second(int $a, MockObject $m): void {}
    public function __construct(MockObject $m = null) {}
}

class BaseTest extends \PHPUnit\Framework\TestCase
{
    protected function parentTake(MockObject $m): void {}
}
