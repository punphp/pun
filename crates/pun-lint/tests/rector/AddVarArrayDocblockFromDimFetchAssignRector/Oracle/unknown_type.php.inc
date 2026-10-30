<?php

final class UnknownType
{
    private array $items = [];

    public function add(): void
    {
        $this->items[] = unknown_function();
    }
}
-----
<?php

final class UnknownType
{
    /**
     * @var mixed[]
     */
    private array $items = [];

    public function add(): void
    {
        $this->items[] = unknown_function();
    }
}
