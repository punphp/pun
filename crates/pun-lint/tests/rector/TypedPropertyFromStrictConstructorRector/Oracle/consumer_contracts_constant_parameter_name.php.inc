<?php

const CONSUMER_VALUE = 1;

final class ConsumerConstantParameterCollision
{
    private $value;

    public function __construct(string $CONSUMER_VALUE)
    {
        $this->value = CONSUMER_VALUE;
    }
}

final class ConsumerConstantDifferentParameter
{
    private $value;

    public function __construct(string $other)
    {
        $this->value = CONSUMER_VALUE;
    }
}
-----
<?php

const CONSUMER_VALUE = 1;

final class ConsumerConstantParameterCollision
{
    private string $value;

    public function __construct(string $CONSUMER_VALUE)
    {
        $this->value = CONSUMER_VALUE;
    }
}

final class ConsumerConstantDifferentParameter
{
    private int $value;

    public function __construct(string $other)
    {
        $this->value = CONSUMER_VALUE;
    }
}
