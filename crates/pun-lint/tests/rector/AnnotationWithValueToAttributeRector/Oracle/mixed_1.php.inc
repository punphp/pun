<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * Text.
 *
 * @uses \Foo\Bar
 * @backupGlobals maybe
 * @dataProvider it's
 * @uses(x)
 * @author me
 */
final class SomeTest extends TestCase
{
    /**
     * @dataProvider provideA
     * @uses Baz::qux
     * @backupGlobals Disabled
     * @dataProvider
     */
    #[\Foo]
    public function testA()
    {
    }

    /** @uses X */
    public function testB() {}

    public function testC() {}
}

$a = new class extends TestCase {
    /**
     * @dataProvider three
     * @uses Y
     */
    public function testC() {}
};
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * Text.
 *
 * @uses(x)
 * @author me
 */
#[\PHPUnit\Framework\Attributes\BackupGlobals(null)]
#[\PHPUnit\Framework\Attributes\DataProvider("it's")]
#[\PHPUnit\Framework\Attributes\UsesClass('\Foo\Bar')]
#[\PHPUnit\Framework\Attributes\UsesClass(Baz::qux)]
#[\PHPUnit\Framework\Attributes\UsesClass('X')]
final class SomeTest extends TestCase
{
    #[\Foo]
    #[\PHPUnit\Framework\Attributes\BackupGlobals(false)]
    #[\PHPUnit\Framework\Attributes\DataProvider('provideA')]
    #[\PHPUnit\Framework\Attributes\DataProvider('')]
    public function testA()
    {
    }

    public function testB() {}

    public function testC() {}
}

$a = new #[\PHPUnit\Framework\Attributes\UsesClass('Y')] class extends TestCase {
    #[\PHPUnit\Framework\Attributes\DataProvider('three')]
    public function testC() {}
};
