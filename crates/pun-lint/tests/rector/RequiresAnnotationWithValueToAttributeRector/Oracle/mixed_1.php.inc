<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * Text.
 *
 * @requires PHP
 * @requires foo bar
 * @requires OS  Linux
 * @requires PHP 8.1
 * @requires PHP 8.1
 * @requires extension
 * @requires function Foo::bar::baz
 * @requires function a b
 * @requires setting x
 * @requires PHPUnit it's
 * @requires(x)
 */
final class SomeTest extends TestCase
{
    /**
     * @requires foo bar
     */
    public function testA()
    {
    }

    /**
     * @requires PHP 8.1
     * @requires foo bar
     * @author me
     */
    #[\Foo]
    public function testB()
    {
    }

    /**
     * @requires OSFAMILY Linux
     * @requires function Foo::class
     * @requires extension a b c
     */
    public static function testC()
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * Text.
 *
 * @requires foo bar
 * @requires(x)
 */
#[\PHPUnit\Framework\Attributes\RequiresPhp(null)]
#[\PHPUnit\Framework\Attributes\RequiresOperatingSystem(' Linux')]
#[\PHPUnit\Framework\Attributes\RequiresPhp('>= 8.1')]
#[\PHPUnit\Framework\Attributes\RequiresPhpExtension('')]
#[\PHPUnit\Framework\Attributes\RequiresMethod(Foo::class, 'bar', 'baz')]
#[\PHPUnit\Framework\Attributes\RequiresFunction('a b')]
#[\PHPUnit\Framework\Attributes\RequiresSetting('x')]
#[\PHPUnit\Framework\Attributes\RequiresPhpunit("it's")]
final class SomeTest extends TestCase
{
    /**
     * @requires foo bar
     */
    public function testA()
    {
    }

    /**
     * @requires foo bar
     * @author me
     */
    #[\Foo]
    #[\PHPUnit\Framework\Attributes\RequiresPhp('>= 8.1')]
    public function testB()
    {
    }

    #[\PHPUnit\Framework\Attributes\RequiresOperatingSystemFamily('Linux')]
    #[\PHPUnit\Framework\Attributes\RequiresMethod(Foo::class, 'class')]
    #[\PHPUnit\Framework\Attributes\RequiresPhpExtension('a', 'b c')]
    public static function testC()
    {
    }
}
