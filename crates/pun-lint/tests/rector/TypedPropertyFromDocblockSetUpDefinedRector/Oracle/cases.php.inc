<?php

namespace App;

use DateTime as Clock;
use PHPUnit\Framework\TestCase;

final class DocTest extends TestCase
{
    /** @var int */
    private $int;
    /** @var string|null */
    private $nullable;
    /** @var int|string */
    private $union;
    /** @var int[] */
    private $list;
    /** @var Clock */
    private $aliased;
    /** @var Clock The clock */
    private $described;
    /** @var callable */
    private $callable;
    /** @var mixed */
    private $mixed;
    /** @var \Closure */
    private $closure;
    /** @var bool */
    private $bool;
    /** @var self */
    private $self;
    /** @var static */
    private $static2;
    /** @var $this */
    private $this2;
    /** @var self|null */
    private $selfNull;
    /** @var Missing */
    private $missing;
    /** @var int */
    private static $static;
    /** @var int */
    protected $protected;
    /** @var int */
    private $notAssigned;
    /** @var int */
    private $a, $b;
    /** @var int */
    private $sometimes;

    protected function setUp(): void
    {
        $this->int = 1;
        $this->nullable = null;
        $this->union = 1;
        $this->list = [];
        $this->aliased = new Clock();
        $this->described = new Clock();
        $this->callable = 'strlen';
        $this->mixed = 1;
        $this->closure = fn () => 1;
        $this->bool = true;
        $this->self = $this;
        $this->static2 = $this;
        $this->this2 = $this;
        $this->selfNull = $this;
        $this->missing = null;
        self::$static = 1;
        $this->protected = 1;
        $this->a = 1;
        if (rand()) {
            $this->sometimes = 1;
        }
    }
}

final class NotATest
{
    /** @var int */
    private $int;

    protected function setUp(): void
    {
        $this->int = 1;
    }
}

final class NoSetUp extends TestCase
{
    /** @var int */
    private $int;

    public function __construct()
    {
        $this->int = 1;
    }
}

final class Doc84Test extends TestCase
{
    /** @var ?self */
    private $nullableSelf;
    /** @var int */
    private $hooked { get => $this->hooked; }

    protected function setUp(): void
    {
        $this->nullableSelf = $this;
        $this->hooked = 1;
    }
}
-----
<?php

namespace App;

use DateTime as Clock;
use PHPUnit\Framework\TestCase;

final class DocTest extends TestCase
{
    private int $int;
    private ?string $nullable;
    /** @var int|string */
    private $union;
    /** @var int[] */
    private array $list;
    private \DateTime $aliased;
    /** @var Clock The clock */
    private \DateTime $described;
    /** @var callable */
    private $callable;
    /** @var mixed */
    private $mixed;
    private \Closure $closure;
    private bool $bool;
    /** @var self */
    private self $self;
    /** @var static */
    private self $static2;
    /** @var $this */
    private self $this2;
    /** @var self|null */
    private ?self $selfNull;
    /** @var Missing */
    private $missing;
    /** @var int */
    private static $static;
    /** @var int */
    protected $protected;
    /** @var int */
    private $notAssigned;
    /** @var int */
    private $a, $b;
    /** @var int */
    private $sometimes;

    protected function setUp(): void
    {
        $this->int = 1;
        $this->nullable = null;
        $this->union = 1;
        $this->list = [];
        $this->aliased = new Clock();
        $this->described = new Clock();
        $this->callable = 'strlen';
        $this->mixed = 1;
        $this->closure = fn () => 1;
        $this->bool = true;
        $this->self = $this;
        $this->static2 = $this;
        $this->this2 = $this;
        $this->selfNull = $this;
        $this->missing = null;
        self::$static = 1;
        $this->protected = 1;
        $this->a = 1;
        if (rand()) {
            $this->sometimes = 1;
        }
    }
}

final class NotATest
{
    /** @var int */
    private $int;

    protected function setUp(): void
    {
        $this->int = 1;
    }
}

final class NoSetUp extends TestCase
{
    /** @var int */
    private $int;

    public function __construct()
    {
        $this->int = 1;
    }
}

final class Doc84Test extends TestCase
{
    /** @var ?self */
    private ?self $nullableSelf;
    private int $hooked { get => $this->hooked; }

    protected function setUp(): void
    {
        $this->nullableSelf = $this;
        $this->hooked = 1;
    }
}
