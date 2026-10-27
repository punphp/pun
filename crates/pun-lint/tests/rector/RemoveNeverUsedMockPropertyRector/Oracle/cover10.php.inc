<?php

namespace App\Tests\Cover10;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

abstract class BaseSetUpTest extends TestCase
{
    private MockObject $mock;

    abstract protected function setUp(): void;
}

abstract class UnusedMockWithAbstractTest extends TestCase
{
    private MockObject $unused;

    protected function setUp(): void
    {
        $this->unused = $this->createMock(\stdClass::class);
        $this->unused->method('x');
    }

    public function testStatic(): void
    {
        $x = new \stdClass();
        $y = $x::$unused;
    }

    abstract protected function other();
}

trait ReadsStatics
{
    public function read()
    {
        return Other::$value;
    }
}

final class LocalTest extends TestCase
{
    use ReadsStatics;

    private $value;

    protected function setUp(): void
    {
        $this->value = 'a';
        $this->value .= 'b';
    }

    #[DataProvider('provide')]
    public function testIt(int $first): void
    {
    }

    public static function provide(): array
    {
        if (rand()) {
            return;
        }
        return [
            'name' => [1],
        ];
    }
}
-----
<?php

namespace App\Tests\Cover10;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

abstract class BaseSetUpTest extends TestCase
{
    private MockObject $mock;

    abstract protected function setUp(): void;
}

abstract class UnusedMockWithAbstractTest extends TestCase
{
    protected function setUp(): void
    {
    }

    public function testStatic(): void
    {
        $x = new \stdClass();
        $y = $x::$unused;
    }

    abstract protected function other();
}

trait ReadsStatics
{
    public function read()
    {
        return Other::$value;
    }
}

final class LocalTest extends TestCase
{
    use ReadsStatics;

    private $value;

    protected function setUp(): void
    {
        $this->value = 'a';
        $this->value .= 'b';
    }

    #[DataProvider('provide')]
    public function testIt(int $first): void
    {
    }

    public static function provide(): array
    {
        if (rand()) {
            return;
        }
        return [
            'name' => [1],
        ];
    }
}
