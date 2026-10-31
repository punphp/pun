<?php

abstract class ConsumerAbstractConstructor
{
    private $value;

    abstract public function __construct(int $value);
}

final class ConsumerConcreteConstructor
{
    private $value;

    public function __construct(int $value)
    {
        $this->value = $value;
    }
}
-----
<?php

abstract class ConsumerAbstractConstructor
{
    private $value;

    abstract public function __construct(int $value);
}

final class ConsumerConcreteConstructor
{
    private int $value;

    public function __construct(int $value)
    {
        $this->value = $value;
    }
}
