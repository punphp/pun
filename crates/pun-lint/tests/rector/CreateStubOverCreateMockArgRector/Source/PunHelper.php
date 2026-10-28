<?php

namespace Pun\Ns2;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;

/**
 * @method void magicMock(MockObject $m)
 * @method void magicAny($m)
 */
class Helper
{
    public function __construct($m = null, ?MockObject $n = null) {}
    public function take(MockObject $m): void {}
    public static function takeStatic(MockObject $m): void {}
    /** @param MockObject $m */
    public function docOnly($m): void {}
    public function nullable(?MockObject $m): void {}
    public function untyped($m): void {}
    public function stub(Stub $m): void {}
    public function intersection(MockObject&\stdClass $m): void {}
    public function variadic(MockObject ...$m): void {}
    public function second(int $a, MockObject $m): void {}
    /**
     * @template T
     * @param T $m
     */
    public function generic($m): void {}
    public function __call($name, $args) {}
}

class MockHolder
{
    public function __construct(public MockObject $m) {}
}

class NoConstructor
{
}

class Other
{
    public function take(MockObject $m): void {}
}

class BaseTest extends \PHPUnit\Framework\TestCase
{
    protected function parentTake(MockObject $m): void {}
    protected function parentAny($m): void {}
}
