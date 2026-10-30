<?php

namespace QualityPropertyContracts;

final class DocUnions
{
    /** @var int|string */
    private $integer;
    /** @var float|string */
    private $float;
    /** @var bool|string */
    private $boolean;
    /** @var array|int */
    private $array;
    /** @var \Closure|string */
    private $closure;
    /** @var array */
    private $iterable;
    /** @var \stdClass */
    private $object;
    /** @var \DateTime */
    private $differentObject;
    /** @var float */
    private $wrongScalar;
    /** @var int */
    private $wrongBoolean;

    public function __construct(int $integer, float $float, bool $boolean, \Closure $closure, iterable $iterable, object $object)
    {
        $this->integer = $integer;
        $this->float = $float;
        $this->boolean = $boolean;
        $this->array = [];
        $this->closure = $closure;
        $this->iterable = $iterable;
        $this->object = $object;
        $this->differentObject = new \stdClass();
        $this->wrongScalar = $integer;
        $this->wrongBoolean = $boolean;
    }
}

final class NestedDocUnion
{
    private $value;

    /** @param array{nested:array{value:int|string}}|int $value */
    public function __construct(array|int $value)
    {
        $this->value = $value;
    }
}
-----
<?php

namespace QualityPropertyContracts;

final class DocUnions
{
    private int $integer;
    private float $float;
    private bool $boolean;
    /** @var array|int */
    private array $array;
    /** @var \Closure|string */
    private \Closure $closure;
    /** @var array */
    private iterable $iterable;
    /** @var \stdClass */
    private object $object;
    /** @var \DateTime */
    private \stdClass $differentObject;
    private int $wrongScalar;
    private bool $wrongBoolean;

    public function __construct(int $integer, float $float, bool $boolean, \Closure $closure, iterable $iterable, object $object)
    {
        $this->integer = $integer;
        $this->float = $float;
        $this->boolean = $boolean;
        $this->array = [];
        $this->closure = $closure;
        $this->iterable = $iterable;
        $this->object = $object;
        $this->differentObject = new \stdClass();
        $this->wrongScalar = $integer;
        $this->wrongBoolean = $boolean;
    }
}

final class NestedDocUnion
{
    private $value;

    /** @param array{nested:array{value:int|string}}|int $value */
    public function __construct(array|int $value)
    {
        $this->value = $value;
    }
}
