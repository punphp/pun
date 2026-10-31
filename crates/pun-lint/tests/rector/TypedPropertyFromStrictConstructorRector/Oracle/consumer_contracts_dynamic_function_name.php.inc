<?php

function consumer_factory(): int
{
    return 1;
}

final class ConsumerDynamicFunctionName
{
    private $value;

    public function __construct(string $factory)
    {
        $factory = static fn(): int => 1;
        $this->value = $factory();
    }
}

final class ConsumerNamedFunctionCollision
{
    private $value;

    public function __construct(string $consumer_factory)
    {
        $this->value = consumer_factory();
    }
}

final class ConsumerNamedFunctionDifferentParameter
{
    private $value;

    public function __construct(string $other)
    {
        $this->value = consumer_factory();
    }
}
-----
<?php

function consumer_factory(): int
{
    return 1;
}

final class ConsumerDynamicFunctionName
{
    private int $value;

    public function __construct(string $factory)
    {
        $factory = static fn(): int => 1;
        $this->value = $factory();
    }
}

final class ConsumerNamedFunctionCollision
{
    private string $value;

    public function __construct(string $consumer_factory)
    {
        $this->value = consumer_factory();
    }
}

final class ConsumerNamedFunctionDifferentParameter
{
    private int $value;

    public function __construct(string $other)
    {
        $this->value = consumer_factory();
    }
}
