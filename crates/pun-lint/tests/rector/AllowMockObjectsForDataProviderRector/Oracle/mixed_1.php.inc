<?php

namespace App\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;
use Symfony\Component\Form\Test\TypeTestCase;

/**
 * Doc.
 */
#[Group('x')]
final class FirstTest extends TypeTestCase
{
}

#[\phpunit\framework\attributes\allowmockobjectswithoutexpectations]
final class SecondTest extends TypeTestCase
{
}

final class ThirdTest extends TestCase
{
}

abstract class FourthTest extends TypeTestCase
{
    #[DataProvider('provide')]
    public function testIt(): void
    {
        $this->createMock(\stdClass::class)->method('x');
    }
}

final class FifthTest extends TestCase
{
    #[DataProvider('provide')]
    public function testIt(): void
    {
        $this->createMock(\stdClass::class)->method('x');
        $fn = function () {
            $mock->method('x');
        };
    }

    /** @test */
    #[DataProvider('provide')]
    public function documented(): void
    {
        $mock->expects($this->once())->method('x');
    }

    #[DataProvider('provide')]
    public function notATest(): void
    {
        $mock->method('x');
    }
}

final class SixthTest extends TestCase
{
    /**
     * @test
     */
    #[DataProvider('provide')]
    public function documented(): void
    {
        $mock->method('x');
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;
use Symfony\Component\Form\Test\TypeTestCase;

/**
 * Doc.
 */
#[Group('x')]
final class FirstTest extends TypeTestCase
{
}

#[\phpunit\framework\attributes\allowmockobjectswithoutexpectations]
final class SecondTest extends TypeTestCase
{
}

final class ThirdTest extends TestCase
{
}

abstract class FourthTest extends TypeTestCase
{
    #[DataProvider('provide')]
    public function testIt(): void
    {
        $this->createMock(\stdClass::class)->method('x');
    }
}

final class FifthTest extends TestCase
{
    #[DataProvider('provide')]
    public function testIt(): void
    {
        $this->createMock(\stdClass::class)->method('x');
        $fn = function () {
            $mock->method('x');
        };
    }

    /** @test */
    #[DataProvider('provide')]
    public function documented(): void
    {
        $mock->expects($this->once())->method('x');
    }

    #[DataProvider('provide')]
    public function notATest(): void
    {
        $mock->method('x');
    }
}

#[\PHPUnit\Framework\Attributes\AllowMockObjectsWithoutExpectations]
final class SixthTest extends TestCase
{
    /**
     * @test
     */
    #[DataProvider('provide')]
    public function documented(): void
    {
        $mock->method('x');
    }
}
