<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, callable(): mixed>
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
     * @var array<int, callable(): mixed>
     */
    private array $items = [];

    /**
     * @return array<int, callable(): mixed>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
