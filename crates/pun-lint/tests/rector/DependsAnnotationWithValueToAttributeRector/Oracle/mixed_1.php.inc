<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function testOne() {}

    /**
     * Doc.
     *
     * @depends testThree
     * @depends TESTONE
     * @depends testTwo
     * @depends clone testOne
     * @depends clone testTwo
     * @depends clone missing
     * @depends Other::class
     * @depends \Foo\Other::testX
     * @depends Other::a::b
     * @depends missing
     * @depends ::x
     * @depends it's::x
     * @depends(testOne)
     */
    #[\Foo]
    public function testTwo(): void
    {
    }

    /** @depends testOne */
    public function testThree() {}

    /**
     * @depends missing
     */
    public function testFour() {}
}

$a = new class extends TestCase {
    public function testA() {}

    /**
     * @depends testA
     */
    public function testB() {}
};
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function testOne() {}

    /**
     * Doc.
     *
     * @depends testTwo
     * @depends clone testTwo
     * @depends clone missing
     * @depends missing
     * @depends ::x
     * @depends(testOne)
     */
    #[\Foo]
    #[\PHPUnit\Framework\Attributes\Depends('testThree')]
    #[\PHPUnit\Framework\Attributes\Depends('TESTONE')]
    #[\PHPUnit\Framework\Attributes\DependsUsingDeepClone('testOne')]
    #[\PHPUnit\Framework\Attributes\DependsOnClass(Other::class)]
    #[\PHPUnit\Framework\Attributes\DependsExternal(\Foo\Other::class, 'testX')]
    #[\PHPUnit\Framework\Attributes\DependsExternal(Other::class, a::b)]
    #[\PHPUnit\Framework\Attributes\DependsExternal("it's::class", 'x')]
    public function testTwo(): void
    {
    }

    #[\PHPUnit\Framework\Attributes\Depends('testOne')]
    public function testThree() {}

    /**
     * @depends missing
     */
    public function testFour() {}
}

$a = new class extends TestCase {
    public function testA() {}

    #[\PHPUnit\Framework\Attributes\Depends('testA')]
    public function testB() {}
};
