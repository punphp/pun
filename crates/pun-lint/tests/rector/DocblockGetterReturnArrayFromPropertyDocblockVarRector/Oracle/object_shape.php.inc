<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, object{a: int}>
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
     * @var array<int, object{a: int}>
     */
    private array $items = [];

    /**
     * @return array<int, object{a: int}>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
