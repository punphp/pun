<?php

final class ConsumerVariadicFallback
{
    private $items;

    public function __construct(int ...$input)
    {
        $this->items = $input;
    }
}

final class ConsumerArrayFallback
{
    private $items;

    public function __construct(array $input)
    {
        $this->items = $input;
    }
}
-----
<?php

final class ConsumerVariadicFallback
{
    private array $items;

    public function __construct(int ...$input)
    {
        $this->items = $input;
    }
}

final class ConsumerArrayFallback
{
    private array $items;

    public function __construct(array $input)
    {
        $this->items = $input;
    }
}
