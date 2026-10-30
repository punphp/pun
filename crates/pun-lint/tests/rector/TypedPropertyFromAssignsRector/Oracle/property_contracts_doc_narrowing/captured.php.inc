<?php

namespace QualityPropertyContracts;

final class NarrowedObject
{
    private $value;

    /** @param \stdClass $value */
    public function __construct(object $value)
    {
        $this->value = $value;
    }
}

final class NarrowedUnion
{
    private $value;

    /** @param array{0:int, key:string}|int $value */
    public function __construct(array|int $value)
    {
        $this->value = $value;
    }
}

final class ListedValues
{
    /** @var list<int> */
    private $value;

    public function __construct(string $text)
    {
        $this->value = explode(',', $text);
    }
}

final class DynamicValue
{
    private $value;

    public function __construct(string $name)
    {
        $this->value = ${$name};
    }
}
-----
<?php

namespace QualityPropertyContracts;

final class NarrowedObject
{
    private $value;

    /** @param \stdClass $value */
    public function __construct(object $value)
    {
        $this->value = $value;
    }
}

final class NarrowedUnion
{
    private $value;

    /** @param array{0:int, key:string}|int $value */
    public function __construct(array|int $value)
    {
        $this->value = $value;
    }
}

final class ListedValues
{
    /** @var list<int> */
    private array $value;

    public function __construct(string $text)
    {
        $this->value = explode(',', $text);
    }
}

final class DynamicValue
{
    private $value;

    public function __construct(string $name)
    {
        $this->value = ${$name};
    }
}
