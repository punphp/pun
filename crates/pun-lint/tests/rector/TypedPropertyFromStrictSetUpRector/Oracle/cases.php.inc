<?php

namespace App;

use PHPUnit\Framework\TestCase;

final class Helper {}

final class MultiTest extends TestCase
{
    private $a, $b;
    private static $s;
    /** @var \DateTime */
    private $mock;
    /** @var \PHPUnit\Framework\MockObject\MockObject */
    private $mock2;
    /** @var \DateTime|null */
    private $mock3;
    /**
     * Some text.
     *
     * @var Helper
     */
    private $mock4;
    private $mock5;
    private $withDefault = 5;
    protected $prot;
    public $pub;
    private $str;
    private $twice;
    private $never;
    private $arr;
    private $obj;
    private string $typed;

    protected function SETUP(): void
    {
        $this->a = 1;
        $this->b = 'x';
        self::$s = 2;
        $this->mock = $this->createMock(\DateTime::class);
        $this->mock2 = $this->createMock(\DateTime::class);
        $this->mock3 = $this->createMock(\DateTime::class);
        $this->mock4 = $this->createMock(\DateTime::class);
        $this->mock5 = $this->createMock(\DateTime::class);
        $this->withDefault = 10;
        $this->prot = 1;
        $this->pub = 1;
        $this->str = 'a';
        $this->twice = 1;
        $this->twice = 'b';
        $this->arr = [1, 2];
        $this->obj = new \stdClass();
        $this->typed = 'a';
    }
}

final class NotATest
{
    private $value;

    public function setUp()
    {
        $this->value = 1000;
    }
}

final class NoSetUp extends TestCase
{
    private $value;

    public function prepare()
    {
        $this->value = 1000;
    }
}
-----
<?php

namespace App;

use PHPUnit\Framework\TestCase;

final class Helper {}

final class MultiTest extends TestCase
{
    private int $a, $b;
    private static int $s;
    /** @var (\PHPUnit\Framework\MockObject\MockObject & \DateTime) */
    private \PHPUnit\Framework\MockObject\MockObject $mock;
    /** @var \PHPUnit\Framework\MockObject\MockObject */
    private \PHPUnit\Framework\MockObject\MockObject $mock2;
    /** @var \DateTime|null */
    private \PHPUnit\Framework\MockObject\MockObject $mock3;
    /**
     * Some text.
     *
     * @var (\PHPUnit\Framework\MockObject\MockObject & \App\Helper)
     */
    private \PHPUnit\Framework\MockObject\MockObject $mock4;
    private \PHPUnit\Framework\MockObject\MockObject $mock5;
    private int $withDefault = 5;
    protected $prot;
    public $pub;
    private string $str;
    private $twice;
    private $never;
    private $arr;
    private \stdClass $obj;
    private string $typed;

    protected function SETUP(): void
    {
        $this->a = 1;
        $this->b = 'x';
        self::$s = 2;
        $this->mock = $this->createMock(\DateTime::class);
        $this->mock2 = $this->createMock(\DateTime::class);
        $this->mock3 = $this->createMock(\DateTime::class);
        $this->mock4 = $this->createMock(\DateTime::class);
        $this->mock5 = $this->createMock(\DateTime::class);
        $this->withDefault = 10;
        $this->prot = 1;
        $this->pub = 1;
        $this->str = 'a';
        $this->twice = 1;
        $this->twice = 'b';
        $this->arr = [1, 2];
        $this->obj = new \stdClass();
        $this->typed = 'a';
    }
}

final class NotATest
{
    private $value;

    public function setUp()
    {
        $this->value = 1000;
    }
}

final class NoSetUp extends TestCase
{
    private $value;

    public function prepare()
    {
        $this->value = 1000;
    }
}
