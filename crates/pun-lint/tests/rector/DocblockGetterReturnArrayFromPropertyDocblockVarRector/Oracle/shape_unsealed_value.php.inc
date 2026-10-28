<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, array{a: int, ...<int>}>
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
     * @var array<int, array{a: int, ...<int>}>
     */
    private array $items = [];

    /**
     * @return array<int, array{a: int, ...<int>}>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
