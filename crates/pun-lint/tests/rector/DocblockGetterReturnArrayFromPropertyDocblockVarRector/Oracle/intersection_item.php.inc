<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, Countable&Traversable>
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, Countable&Traversable>
     */
    private array $items = [];

    /**
     * @return array<int, Countable&Traversable>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
